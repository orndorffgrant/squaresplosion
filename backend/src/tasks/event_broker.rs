use async_std::task;

use futures::{
    channel::mpsc,
    select,
    stream::StreamExt,
    sink::SinkExt,
    FutureExt,
};

use log::{info, trace, error};

use std::collections::hash_map::{
    Entry,
    HashMap,
};

use crate::{
    internal_messages::Event,
    state,
    tasks,
    types,
    ws_messages,
};

pub async fn run(incoming_events: types::ChannelReceiver<Event>) -> types::ServerResult<()> {
    trace!("event_broker: starting up");
    let (disconnect_sender, mut disconnect_receiver) = mpsc::unbounded::<usize>();
    let mut rooms: HashMap<String, state::SquareRoomState> = HashMap::new();
    let mut conns: HashMap<usize, String> = HashMap::new();

    let mut incoming_events = incoming_events.fuse();

    loop {
        trace!("event_broker: waiting");
        let event = select! {
            event = incoming_events.next().fuse() => match event {
                None => break,
                Some(event) => event,
            },
            disconnect = disconnect_receiver.next().fuse() => {
                // TODO is this unwrap safe?
                let disconnected_conn = disconnect.unwrap();
                trace!("event_broker: received disconnect from {}", disconnected_conn);

                let room_name = match conns.get(&disconnected_conn) {
                    Some(r_ref) => r_ref,
                    None => {
                        error!("event_broker: cant find room for conn_id {}", disconnected_conn);
                        continue;
                    }
                };
                let room = match rooms.get_mut(room_name) {
                    Some(r_ref) => r_ref,
                    None => {
                        error!("event_broker: cant find room for conn_id {}", disconnected_conn);
                        continue;
                    }
                };

                trace!("event_broker: removing player");
                room.remove_player(disconnected_conn);

                if room.player_senders.len() == 0 {
                    trace!("event_broker: room {} empty, removing room", room_name);
                    rooms.remove(room_name);
                }
                
                trace!("event_broker: removing conn");
                conns.remove(&disconnected_conn);
                continue;
            },
        };
        trace!("event_broker: received event");

        match event {
            Event::NewConnection{
                conn_id,
                player_id,
                player_name,
                room_name,
                x,
                y,
                new_room,
                mut ws_sender,
                shutdown_receiver
            } => {
                trace!("event_broker: received new conn from conn {} player {} for room {}", conn_id, player_id, room_name);
                let room_intermediate = rooms.entry(room_name.to_string());
                let room = {
                    if new_room {
                        match room_intermediate {
                            Entry::Vacant(_) => {
                                info!("event_broker: received new_room message for an existing room, terminating connection");
                                match ws_sender.close().await {
                                    Ok(_) => {
                                        return Ok(());
                                    },
                                    Err(e) => {
                                        error!("failed to close ws_sender: {}", e);
                                        return Ok(());
                                    },
                                }
                            }
                            Entry::Occupied(e) => e.into_mut(),
                        }
                    } else {
                        room_intermediate.or_insert_with(|| {
                            trace!("event_broker: room {} doesn't exist, creating", room_name);
                            state::SquareRoomState::new(room_name.as_str())
                        })
                    }
                };

                trace!("event_broker: adding player {} to room {}", player_id, room_name);
                let (conn_outgoing_sender, conn_outgoing_receiver) = mpsc::unbounded();
                room.add_player(conn_id, player_id.as_str(), player_name.as_str(), x, y, conn_outgoing_sender);

                trace!("event_broker: saving room {} for conn {}", room_name, conn_id);
                conns.insert(conn_id, room_name);

                trace!("event_broker: spawning conn_writer for conn {}", conn_id);
                task::spawn(tasks::conn_writer::run(conn_id, ws_sender, disconnect_sender.clone(), conn_outgoing_receiver, shutdown_receiver));

                trace!("event_broker: sending player {} room state", player_id);
                let ref mut senders = room.player_senders;
                for player_sender in senders {
                    if player_sender.conn_id == conn_id {
                        match player_sender.sender.send(ws_messages::RoomStateMessageEvent{
                            player_state: room.player_state.clone(),
                        }).await {
                            Ok(_) => {},
                            Err(e) => {
                                error!("event_broker: {}", e);
                            }
                        };
                        break;
                    }
                }
            },
            Event::PlayerMove { from_id, player_id, x, y } => {
                trace!("event_broker: received move from conn {} for player {}", from_id, player_id);
                let room_name = match conns.get(&from_id) {
                    Some(r_ref) => r_ref,
                    None => {
                        error!("event_broker: cant find room for conn_id {}", from_id);
                        continue;
                    }
                };
                let room = match rooms.get_mut(room_name) {
                    Some(r_ref) => r_ref,
                    None => {
                        error!("event_broker: cant find room for conn_id {}", from_id);
                        continue;
                    }
                };
                trace!("event_broker: updating player {} location in room {}", player_id, room_name);
                room.update_player(&player_id, x, y);

                trace!("event_broker: broadcasting room {} state", room_name);
                let ref mut senders = room.player_senders;
                for player_sender in senders {
                    match player_sender.sender.send(ws_messages::RoomStateMessageEvent{
                        player_state: room.player_state.clone(),
                    }).await {
                        Ok(_) => {},
                        Err(e) => {
                            error!("event_broker: {}", e);
                        }
                    };
                }
            },
        }
    }

    trace!("event_broker: dropping conns and disconnect_sender");
    drop(conns);
    drop(disconnect_sender);
    trace!("event_broker: awaiting all disconnects");
    while let Some(_id) = disconnect_receiver.next().await {
    }

    trace!("event_broker: shutting down");
    Ok(())
}
