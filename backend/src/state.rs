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
    pub room_state: HashMap<u32, HashMap<u32, CellState>>,
}

impl SquareRoomState {
    pub fn new(name: &str) -> SquareRoomState {
        SquareRoomState{
            name: name.to_string(),
            player_senders: Vec::new(),
            conn_player_map: HashMap::new(),
            player_state: HashMap::new(),
            room_state: HashMap::new(),
        }
    }
    fn update_cell(&mut self, id: &str, x: u32, y: u32, always_update: bool) -> &SquareRoomState {
        // TODO unwraps all over
        let player_state = self.player_state.get_mut(id).unwrap();
        let mut to_kill = None;
        if player_state.alive
            && (
                always_update
                || (player_state.x == x - 25 && player_state.y == y)
                || (player_state.x == x + 25 && player_state.y == y)
                || (player_state.x == x && player_state.y == y - 25)
                || (player_state.x == x && player_state.y == y + 25)
            )
        {

            // remove them from old location
            let old_row_opt = self.room_state.get_mut(&player_state.x);
            if let Some(old_row) = old_row_opt {
                let old_cell_opt = old_row.get_mut(&player_state.y);
                if let Some(old_cell) = old_cell_opt {
                    if old_cell.player_id == Some(id.to_string()) {
                        old_cell.player_id = None;
                    }
                }
            }

            // add them to new location
            let row = self.room_state.entry(x).or_insert_with(|| {
                HashMap::new()
            });
            let cell = row.entry(y).or_insert_with(|| {
                CellState{
                    player_id: None,
                }
            });
            let id_string = id.to_string();
            // if cell has another player, kill them
            if let Some(ref other_player_id) = cell.player_id {
                if *other_player_id != id_string {
                    to_kill = Some(other_player_id.clone());
                }
            }
            cell.player_id = Some(id.to_string());

            player_state.x = x;
            player_state.y = y;
        }
        if let Some(to_kill_id) = to_kill {
            let to_kill_player_state_opt = self.player_state.get_mut(&to_kill_id);
            if let Some(to_kill_player_state) = to_kill_player_state_opt {
                to_kill_player_state.alive = false;
            }
        }
        self
    }
    pub fn add_player(&mut self, conn_id: usize, id: &str, name: &str, x: u32, y: u32, sender: types::ChannelSender<ws_messages::RoomStateMessageEvent>) {
        self.player_senders.push(PlayerSender{
            conn_id,
            sender,
        });
        self.conn_player_map.insert(conn_id, id.to_string());
        self.player_state.insert(id.to_string(), PlayerState{
            id: id.to_string(),
            name: name.to_string(),
            score: 1,
            x,
            y,
            alive: true,
        });
        self.update_cell(id, x, y, true);
    }
    pub fn update_player(&mut self, id: &str, x: u32, y: u32) {
        self.update_cell(id, x, y, false);
        let curr_player_state = self.player_state.get_mut(id).unwrap();
        curr_player_state.score += 1;
    }
    pub fn remove_player(&mut self, conn_id: usize) {
        // TODO this unwrap
        let player_id = self.conn_player_map.get(&conn_id).unwrap();
        self.player_senders.retain(|s| s.conn_id != conn_id);
        self.player_state.remove(player_id);
        self.conn_player_map.remove(&conn_id);
    }
}
