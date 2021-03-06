use serde::{Deserialize, Serialize};

use std::{collections::hash_map::HashMap, convert::TryInto};

use crate::{types, ws_messages};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub id: String,
    pub name: String,
    pub score: u64,
    pub x: u32,
    pub y: u32,
    pub alive: bool,
}

pub struct CellState {
    pub player_id: Option<String>,
}

pub struct PlayerSender {
    pub conn_id: usize,
    pub sender: types::ChannelSender<ws_messages::RoomStateMessageEvent>,
}

pub struct SquareRoomState {
    pub name: String,
    pub player_senders: Vec<PlayerSender>,
    pub conn_player_map: HashMap<usize, String>,
    pub player_state: HashMap<String, PlayerState>,
    pub room_state: Vec<Vec<CellState>>,
}

impl SquareRoomState {
    pub fn new(name: &str) -> SquareRoomState {
        SquareRoomState {
            name: name.to_string(),
            player_senders: Vec::new(),
            conn_player_map: HashMap::new(),
            player_state: HashMap::new(),
            room_state: vec![
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
                vec![
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                    CellState { player_id: None },
                ],
            ],
        }
    }
    fn update_cell(&mut self, id: &str, x: u32, y: u32, always_update: bool) -> &SquareRoomState {
        // TODO unwraps all over
        let player_state = self.player_state.get_mut(id).unwrap();
        let mut to_kill = None;
        if player_state.alive
            && (always_update || player_state.x != x || player_state.y != y)
            && (x <= 775 && y <= 475)
        {
            // converting u32 to usize is safe as long as we run on a 32 bit or 64 bit machine
            let old_x_index: usize = (player_state.x / 25).try_into().unwrap();
            let old_y_index: usize = (player_state.y / 25).try_into().unwrap();
            let new_x_index: usize = (x / 25).try_into().unwrap();
            let new_y_index: usize = (y / 25).try_into().unwrap();

            // remove them from old location
            self.room_state[old_y_index][old_x_index].player_id = None;

            let cell = &mut self.room_state[new_y_index][new_x_index];
            // if cell has another player, kill them
            if let Some(ref other_player_id) = cell.player_id {
                if !other_player_id.eq(id) {
                    to_kill = Some(other_player_id.clone())
                }
            }

            // add them to new location
            cell.player_id = Some(id.to_string());

            player_state.x = x;
            player_state.y = y;

            // increment score
            player_state.score += 1;
        }
        if let Some(to_kill_id) = to_kill {
            let to_kill_player_state_opt = self.player_state.get_mut(&to_kill_id);
            if let Some(to_kill_player_state) = to_kill_player_state_opt {
                to_kill_player_state.alive = false;
            }
        }
        self
    }
    pub fn add_player(
        &mut self,
        conn_id: usize,
        id: &str,
        name: &str,
        x: u32,
        y: u32,
        sender: types::ChannelSender<ws_messages::RoomStateMessageEvent>,
    ) {
        self.player_senders.push(PlayerSender { conn_id, sender });
        self.conn_player_map.insert(conn_id, id.to_string());
        self.player_state.insert(
            id.to_string(),
            PlayerState {
                id: id.to_string(),
                name: name.to_string(),
                score: 0,
                x,
                y,
                alive: true,
            },
        );
        self.update_cell(id, x, y, true);
    }
    pub fn update_player(&mut self, id: &str, x: u32, y: u32) {
        self.update_cell(id, x, y, false);
    }
    pub fn remove_player(&mut self, conn_id: usize) {
        // TODO this unwrap
        let player_id = self.conn_player_map.get(&conn_id).unwrap();
        self.player_senders.retain(|s| s.conn_id != conn_id);
        self.player_state.remove(player_id);
        self.conn_player_map.remove(&conn_id);
    }
}
