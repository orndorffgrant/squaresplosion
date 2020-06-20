use async_tungstenite::tungstenite::protocol::Message;

use futures::{
    select,
    sink::SinkExt,
    stream::{
        StreamExt,
    },
    FutureExt,
};

use log::{trace, error};

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
    trace!("conn_writer {}: starting up", id);

    loop {
        trace!("conn_writer {}: waiting", id);
        select! {
            msg = messages.next().fuse() => match msg {
                Some(msg) => {
                    trace!("conn_writer {}: sending message", id);
                    let json_msg = serde_json::to_string(&msg)?;
                    match ws_sender.send(Message::Text(json_msg)).await {
                        Ok(m) => m,
                        Err(e) => {
                            error!("conn_writer: {}", e);
                            return Ok(())
                        }
                    }
                },
                None => {
                    trace!("conn_writer {}: None message received", id);
                    break;
                },
            },
            void = shutdown.next().fuse() => match void {
                Some(void) => {
                    trace!("conn_writer {}: shutdown received", id);
                    match void {}
                },
                None => break,
            },
        }
    }

    trace!("conn_writer {}: sending disconnect", id);
    // TODO is this unwrap safe?
    match disconnect_sender.send(id).await {
        Ok(_) => {},
        Err(e) => {
            error!("conn_writer {}: error sending disconnect: {}", id, e);
        },
    };
    trace!("conn_writer {}: shutting down", id);
    Ok(())
}
