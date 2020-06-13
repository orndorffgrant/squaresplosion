use async_std::net::TcpStream;

use async_tungstenite::{
    tungstenite::protocol::Message,
    accept_async,
};

use futures::{
    channel::mpsc,
    sink::SinkExt,
    stream::{
        StreamExt,
    },
};

use log::{info, trace, error};

use crate::{
    internal_messages::Event,
    types,
    ws_messages::{
        PlayerMoveMessage,
        JoinRoomMessage,
    },
};

pub async fn run(
    stream: TcpStream,
    mut event_broker: types::ChannelSender<Event>,
    conn_id: usize,
) -> types::ServerResult<()> {
    trace!("conn_reader {}: starting up", conn_id);
    let peer_addr = match stream.peer_addr() {
        Ok(p) => p,
        Err(e) => {
            error!("conn_reader {}: {}", conn_id, e);
            return Ok(())
        }
    };
    let ws_stream = match accept_async(stream).await {
        Ok(s) => s,
        Err(e) => {
            error!("conn_reader {}: {}", conn_id, e);
            return Ok(())
        }
    };
    info!("conn_reader {}: new conn from {}", conn_id, peer_addr);

    let (outgoing, mut incoming) = ws_stream.split();

    let (_connection_shutdown_sender, connection_shutdown_receiver) = mpsc::unbounded::<types::Void>();

    trace!("conn_reader {}: waiting for room join msg", conn_id);
    let room_join_msg: Message = match incoming.next().await
     {
        Some(m) => m?,
        None => {
            error!("conn_reader {}: connection didn't send room join msg", conn_id);
            return Ok(());
        },
    };

    trace!("conn_reader {}: parsing room join msg", conn_id);
    let room_join: JoinRoomMessage = match serde_json::from_str(room_join_msg.to_string().as_str()) {
        Ok(msg) => msg,
        Err(e) => {
            error!("conn_reader {}: failed to parse room msg: {}", conn_id, e);
            return Ok(())
        }
    };

    trace!("conn_reader {}: player {} joining room {}", conn_id, room_join.player_name, room_join.room_name);

    let player_id = room_join.id;

    match event_broker.send(Event::NewConnection{
        conn_id,
        player_id: player_id.clone(),
        player_name: room_join.player_name,
        room_name: room_join.room_name,
        x: room_join.x,
        y: room_join.y,
        ws_sender: outgoing,
        shutdown_receiver: connection_shutdown_receiver,
    }).await {
        Ok(h) => h,
        Err(e) => {
            error!("conn_reader {}: event broker down {}", conn_id, e);
            return Ok(())
        }
    };

    trace!("conn_reader {}: listening for player move messages", conn_id);

    while let Some(msg) = incoming.next().await {
        let msg = msg?;
        // TODO we get here on a disconnect and presumably fail parsing which ends the task
        trace!("conn_reader {}: parsing new player move message", conn_id);
        let move_msg: PlayerMoveMessage = serde_json::from_str(msg.to_string().as_str())?;
        match event_broker.send(Event::PlayerMove{
            from_id: conn_id,
            player_id: player_id.clone(),
            x: move_msg.x,
            y: move_msg.y,
        }).await {
            Ok(h) => h,
            Err(e) => {
                error!("conn_reader {}: event broker down {}", conn_id, e);
                return Ok(())
            }
        }
    }

    trace!("conn_reader {}: shutting down", conn_id);
    Ok(())
}
