use std::fs::File;
use std::io::prelude::*;
use std::thread;

use read_process_memory::ProcessHandle;

use crate::globals;
use crate::memory::MemoryModel;
use crate::states::game_state::GameState;
use super::game_state::RoundState;
use globals::Player;

/// Time to sleep in between loop checks
pub const TIME_TO_SLEEP_MS: f64 = 1.0f64 / 120.0f64;

pub struct LoopState<M> {
    round_states: Vec<RoundState>,
    index: usize,

    memory: std::marker::PhantomData<M>,
}

impl<M: MemoryModel> LoopState<M> {
    pub fn new() -> Self {
        //let game_data = builder.build();

        Self {
            /// Pre-allocate 6000 frames of data,
            /// taking into account rage art animations in a Tekken 7 
            /// match using official TWT rules (excluding round-victory inputs)
            round_states: Vec::with_capacity(6000),
            index: 0,

            memory: std::marker::PhantomData
            //replay_mode: replay.is_some(),
        }
    }

    fn previous_state(&self) -> Option<&RoundState> {
        self.round_states.get(self.index - 1)
    }

    fn state(&self) -> &RoundState {
        &self.round_states[self.index]
    }

    /// Sleeps until the frame count has been updated
    fn wait_to_proceed(&self) {
        match (
            self.previous_state().map(|s| s.get_round_frame()),
            self.state().get_round_frame(),
        ) {
            (Some(previous_frame), current_frame) 
            if (previous_frame == current_frame) => {
                // Sleep every 1/120th of a second if we're still in the same frame so that we can save our processing power
                thread::sleep(std::time::Duration::from_secs_f64(TIME_TO_SLEEP_MS));
            }
            _ => (),
        };
    }

    pub fn start(&mut self, handle: ProcessHandle) {
        self.start_capture(handle);
    }

    fn start_capture(&mut self, handle: ProcessHandle) {
        let mut game_state = GameState::<M>::new(handle);

        loop {
            // Determine if we should sleep
            self.wait_to_proceed();
            game_state.update().unwrap();

            let current_round = self.state().get_round();
            let current_frame = self.state().get_round_frame();
            let previous_frame = self.previous_state()
                .map(|s| s.get_round_frame());

            let round_state = game_state.round_state();

            // If we're in the same frame, we'll want to wait until the next
            if previous_frame.is_none() {
                match round_state {
                    Some(state) => self.round_states.push(state),
                    None => ()
                };

                continue;
            }

            let previous_frame = previous_frame.unwrap();
            if previous_frame == current_frame {
                continue;
            }

            // Only update the batch once we have a new frame to advance
            let state = round_state.expect(&format!("Expected game state at frame {}, found None", current_frame));
            self.round_states.push(state);

            // Save match data when the round count changes
            if current_frame < previous_frame {
                let round_result = serde_json::to_string(&self.round_states).unwrap();
                let file_name = format!(
                    "C:/Users/gsala/Documents/Tekken7Replays/{}_vs_{}_{}_{}.json",
                    self.state().get_player_state(Player::One).character().to_string(),
                    self.state().get_player_state(Player::Two).character().to_string(),
                    current_round,
                    uuid::Uuid::new_v4()
                );

                println!("Creating file: {}", file_name);

                let mut file = File::create(file_name).expect("Failed to create file");

                file.write(&round_result.as_bytes())
                    .expect("Failed to write buffer to file");
                file.flush().expect("Failed to flush file");

                self.round_states.clear();
            }
        }
    }
}
