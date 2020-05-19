use async_std::task;

use futures::{
    channel::mpsc,
    select,
    stream::StreamExt,
    sink::SinkExt,
    FutureExt,
};

use std::collections::hash_map::HashMap;

use crate::{
    internal_messages::Event,
    state,
    tasks,
    types,
    ws_messages,
};

pub async fn run(incoming_events: types::ChannelReceiver<Event>) -> types::ServerResult<()> {
    let (disconnect_sender, mut disconnect_receiver) = mpsc::unbounded::<usize>();
    let mut rooms: HashMap<String, state::SquareRoomState> = HashMap::new();
    let mut conns: HashMap<usize, String> = HashMap::new();

    let mut incoming_events = incoming_events.fuse();

    loop {
        let event = select! {
            event = incoming_events.next().fuse() => match event {
                None => break,
                Some(event) => event,
            },
            disconnect = disconnect_receiver.next().fuse() => {
                let disconnected_conn = disconnect.unwrap();
                // TODO check if room is empty
                conns.remove(&disconnected_conn);
                continue;
            },
        };

        match event {
            Event::NewConnection{
                conn_id,
                player_id,
                player_name,
                room_name,
                x,
                y,
                ws_sender,
                shutdown_receiver
            } => {
                let room = rooms.entry(room_name.to_string()).or_insert_with(|| {
                    state::SquareRoomState::new(room_name.as_str())
                });

                let (conn_outgoing_sender, conn_outgoing_receiver) = mpsc::unbounded();
                room.add_player(player_id.as_str(), player_name.as_str(), x, y, conn_outgoing_sender);

                conns.insert(conn_id, room_name);
                task::spawn(tasks::conn_writer::run(conn_id, ws_sender, disconnect_sender.clone(), conn_outgoing_receiver, shutdown_receiver));
            },
            Event::PlayerMove { from_id, player_id, x, y } => {
                let room_name = match conns.get(&from_id) {
                    Some(r_ref) => r_ref,
                    None => {
                        eprintln!("cant find room for conn_id {}", from_id);
                        continue;
                    }
                };
                let room = match rooms.get_mut(room_name) {
                    Some(r_ref) => r_ref,
                    None => {
                        eprintln!("cant find room for conn_id {}", from_id);
                        continue;
                    }
                };
                room.update_player(&player_id, x, y);

                for mut sender in &room.player_senders {
                    sender.send(ws_messages::RoomStateMessageEvent{
                        player_state: room.player_state.clone(),
                        room_state: room.room_state.clone(),
                    }).await?;
                }
            },
        }
    }

    drop(conns);
    drop(disconnect_sender);
    while let Some(_id) = disconnect_receiver.next().await {
    }

    Ok(())
}
