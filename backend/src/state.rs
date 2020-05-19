use serde::{Deserialize, Serialize};

use std::collections::hash_map::HashMap;

use crate::{
    types,
    ws_messages,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub id: String,
    pub name: String,
    pub score: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CellState {
    pub player_id: String,
}

pub struct SquareRoomState {
    pub name: String,
    pub player_senders: Vec<types::ChannelSender<ws_messages::RoomStateMessageEvent>>,
    pub player_state: HashMap<String, PlayerState>,
    pub room_state: HashMap<u32, HashMap<u32, CellState>>,
}

impl SquareRoomState {
    pub fn new(name: &str) -> SquareRoomState {
        SquareRoomState{
            name: name.to_string(),
            player_senders: Vec::new(),
            player_state: HashMap::new(),
            room_state: HashMap::new(),
        }
    }
    fn update_cell(&mut self, id: &str, x: u32, y: u32) -> &SquareRoomState {
        let row = self.room_state.entry(x).or_insert_with(|| {
            HashMap::new()
        });
        let cell = row.entry(y).or_insert_with(|| {
            CellState{
                player_id: id.to_string(),
            }
        });
        cell.player_id = id.to_string();
        self
    }
    pub fn add_player(&mut self, id: &str, name: &str, x: u32, y: u32, sender: types::ChannelSender<ws_messages::RoomStateMessageEvent>) {
        self.player_senders.push(sender);
        self.player_state.insert(id.to_string(), PlayerState{
            id: id.to_string(),
            name: name.to_string(),
            score: 1,
        });
        self.update_cell(id, x, y);
    }
    pub fn update_player(&mut self, id: &str, x: u32, y: u32) {
        self.update_cell(id, x, y);
        let curr_player_state = self.player_state.get_mut(id).unwrap();
        curr_player_state.score += 1;
    }
}
