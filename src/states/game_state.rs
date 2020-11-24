use read_process_memory::*;
use serde::{Deserialize, Serialize};

use crate::globals::{self, Player};
use crate::memory::MemoryModel;
use crate::states::player_state::{PlayerState, PlayerInfo};

#[derive(Clone, Serialize, Deserialize)]
pub struct RoundState {
    round: u8,
    round_frame: u128,

    player_info: (PlayerInfo, PlayerInfo),
    player_state: (PlayerState, PlayerState)
}

impl RoundState {
    pub fn get_round(&self) -> u8 {
        self.round
    }

    pub fn get_round_frame(&self) -> u128 {
        self.round_frame
    }

    pub fn get_player_state(&self, player: Player) -> &PlayerState {
        match player {
            Player::One => &self.player_state.0,
            Player::Two => &self.player_state.1
        }
    }

    /// Sets the frame count for the current round.
    pub fn update_round_frame(&mut self, round_frame: u128) {
        self.round_frame = round_frame;
    }

    pub fn update_player_state<M: MemoryModel>(&mut self, memory: &M, player: Player) {
        match player {
            Player::One => &self.player_state.0.update(memory),
            Player::Two => &self.player_state.1.update(memory)
        };
    }
}

#[derive(Clone)]
pub struct GameState<M> {
    /// Memory of the game
    memory: M,
    /// State of the current round
    pub(crate) state: Option<RoundState>
}

impl<M: MemoryModel> GameState<M> {
    pub fn new(handle: ProcessHandle) -> Self {
        Self {
            memory: M::new(handle),
            state: None
        }
    }

    pub fn start(&mut self) {
        while self.memory.round_frame().unwrap() != 0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(1f64 / 120f64));
        }

        self.state = Some(RoundState {
            round: self.memory.round().unwrap().into(),
            round_frame: self.memory.round_frame().unwrap().into(),

            player_info: (
                PlayerInfo { screen_name: None },
                PlayerInfo { screen_name: None }
            ),
            player_state: (
                PlayerState::new(&self.memory, Player::One),
                PlayerState::new(&self.memory, Player::Two)
            )
        });

        self.update().unwrap();
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let state: &mut RoundState = self.state.as_mut().expect("Update requires `start()` to be called first");

        state.update_round_frame(self.memory.round_frame()?.into());
        state.update_player_state::<M>(&self.memory, Player::One);
        state.update_player_state::<M>(&self.memory, Player::Two);

        println!(
            "Frame: {}\tP1: {}, {} \t P2: {}, {}",
            state.round_frame,
            globals::InputButton::from(state.player_state.0.input_attack as usize).to_str(),
            globals::InputDirection::from(state.player_state.0.input_direction as usize).to_str(),
            globals::InputButton::from(state.player_state.1.input_attack as usize).to_str(),
            globals::InputDirection::from(state.player_state.1.input_direction as usize).to_str()
        );

        Ok(())
    }

    pub fn round_state(&mut self) -> Option<RoundState> {
        let mut round_state = None;
        std::mem::swap(&mut self.state, &mut round_state);
        
        round_state
    }

    pub fn clone_round_state(&self) -> RoundState {
        self.state.as_ref().unwrap().clone()
    }
}
