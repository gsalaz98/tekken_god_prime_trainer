use super::patches::Version;
use crate::memory::MemoryModel;
use crate::states::game_state::GameState;
use crate::states::player_state::{PlayerInfo, PlayerState};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MatchReplay<M: MemoryModel> {
    players: Vec<String>,
    version: Version,
    round: Vec<GameState<M>>,
}

impl<M: MemoryModel> MatchReplay<M> {
    pub fn round_states(&self) -> Vec<GameState<M>> {
        &self.round
    }
}
