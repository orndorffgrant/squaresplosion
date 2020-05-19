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
    let peer_addr = match stream.peer_addr() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(())
        }
    };
    let ws_stream = match accept_async(stream).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(())
        }
    };
    println!("New conn ({}) from {}", conn_id, peer_addr);

    let (outgoing, mut incoming) = ws_stream.split();

    let (_connection_shutdown_sender, connection_shutdown_receiver) = mpsc::unbounded::<types::Void>();

    let room_join_msg: Message = match incoming.next().await
     {
        Some(m) => m?,
        None => {
            eprintln!("connection didn't send room join msg");
            return Ok(());
        },
    };

    let room_join: JoinRoomMessage = serde_json::from_str(room_join_msg.to_string().as_str())?;

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
            eprintln!("event broker down {}", e);
            return Ok(())
        }
    };

    while let Some(msg) = incoming.next().await {
        let msg = msg?;
        let move_msg: PlayerMoveMessage = serde_json::from_str(msg.to_string().as_str())?;
        match event_broker.send(Event::PlayerMove{
            from_id: conn_id,
            player_id: player_id.clone(),
            x: move_msg.x,
            y: move_msg.y,
        }).await {
            Ok(h) => h,
            Err(e) => {
                eprintln!("event broker down {}", e);
                return Ok(())
            }
        }
    }

    Ok(())
}
