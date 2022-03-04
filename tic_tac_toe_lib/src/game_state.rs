use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameState {
    pub board: Vec<Vec<String>>,
    pub next_on_move: String,
    pub finished: bool,
    pub result: String,
}

pub fn deserialize_game_state(s: &str) -> GameState {
    serde_json::from_str(s).unwrap()
}
