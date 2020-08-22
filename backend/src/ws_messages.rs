use serde::{Deserialize, Serialize};

use std::collections::hash_map::HashMap;

use crate::state;

#[derive(Serialize, Deserialize)]
pub struct JoinRoomMessage {
    pub id: String,
    pub player_name: String,
    pub room_name: String,
    pub x: u32,
    pub y: u32,
    pub new_room: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerMoveMessage {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RoomStateMessageEvent {
    pub player_state: HashMap<String, state::PlayerState>,
}
