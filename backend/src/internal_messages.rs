use crate::types;

pub enum Event {
    NewConnection {
        conn_id: usize,
        player_id: String,
        player_name: String,
        room_name: String,
        x: u32,
        y: u32,
        ws_sender: types::WebSocketSender,
        shutdown_receiver: types::ChannelReceiver<types::Void>
    },
    PlayerMove {
        from_id: usize,
        player_id: String,
        x: u32,
        y: u32,
    },
}
