use async_tungstenite::tungstenite::protocol::Message;

use futures::{
    select,
    sink::SinkExt,
    stream::{
        StreamExt,
    },
    FutureExt,
};

use crate::{
    types,
    ws_messages,
};


pub async fn run(
    id:  usize,
    mut ws_sender: types::WebSocketSender,
    mut disconnect_sender: types::ChannelSender<usize>,
    message_receiver: types::ChannelReceiver<ws_messages::RoomStateMessageEvent>,
    shutdown_receiver: types::ChannelReceiver<types::Void>,
) -> types::ServerResult<()> {
    let mut messages = message_receiver.fuse();
    let mut shutdown = shutdown_receiver.fuse();

    loop {
        select! {
            msg = messages.next().fuse() => match msg {
                Some(msg) => {
                    let json_msg = serde_json::to_string(&msg)?;
                    match ws_sender.send(Message::Text(json_msg)).await {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("{}", e);
                            return Ok(())
                        }
                    }
                },
                None => break,
            },
            void = shutdown.next().fuse() => match void {
                Some(void) => match void {},
                None => break,
            },
        }
    }

    disconnect_sender.send(id).await.unwrap();
    Ok(())
}
