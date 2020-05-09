use serde::{Serialize, Deserialize};
use super::patches::Version;
use crate::{memory::MemoryModel, game_state::GameState};

#[derive(Serialize, Deserialize)]
pub struct MatchReplay<M: MemoryModel> {
    players: Vec<String>,
    version: Version,
    round: Vec<GameState<M>>
}

impl<M: MemoryModel> MatchReplay<M> {
    pub fn round_states(&self) -> Vec<GameState<M>> {
        &self.round
    }
}