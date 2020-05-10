use std::fs::File;
use std::io::prelude::*;
use std::thread;

use read_process_memory::ProcessHandle;

use crate::globals;
use crate::memory::MemoryModel;
use crate::replay::player_match::MatchReplay;
use crate::states::game_state::GameState;
use crate::states::player_state::{PlayerInfo, PlayerState};

/// Time to sleep in between loop checks
pub const TIME_TO_SLEEP_MS: f64 = 1.0f64 / 120.0f64;

pub struct LoopState<M: MemoryModel> {
    round_states: Vec<GameState<M>>,
    index: usize,
    //replay_mode: bool,
}

impl<M: MemoryModel> LoopState<M> {
    pub fn new(handle: ProcessHandle) -> Self {
        //let game_data = builder.build();

        Self {
            /// Pre-allocate 3600 frames of data (1 minute)
            /// which is the max amount of frames that can exist
            /// in a Tekken 7 match using official TWT rules (excluding round-victory inputs)
            round_states: Vec::with_capacity(3600),
            index: 0,
            //replay_mode: replay.is_some(),
        }
    }

    fn state(&self) -> &GameState<M> {
        &self.round_states[self.index]
    }

    /// Sleeps until the frame count has been updated
    fn wait_to_proceed(&self) {
        match (
            self.state().round_frame_count_previous(),
            self.state().round_frame_count(),
        ) {
            (previous_frame, current_frame) if (previous_frame == current_frame) => {
                // Sleep every 1/120th of a second if we're still in the same frame so that we can save our processing power
                thread::sleep(std::time::Duration::from_secs_f64(TIME_TO_SLEEP_MS));
            }
            _ => (),
        };
    }

    pub fn start(&mut self) {
        //match self.replay_mode {
        //    true => self.start_replay(),
        //    false => self.start_capture(),
        //};
        self.start_capture();
    }

    //fn start_replay(&mut self) {
    //    // Wait until we're in a game to start updating the frames
    //    while let None = self.state().round_frame_count() {
    //        self.wait_to_proceed();
    //        self.state().update();
    //    }

    //    for i in 0..self.round_states.len() {
    //        // Determine if we should sleep. On the first frame
    //        // this is guaranteed to be skipped because the previous frame
    //        // hasn't been set yet, which means we begin the replay the
    //        // moment we know we're in a game
    //        self.wait_to_proceed();

    //        match self.state().round_frame_count() {
    //            Some(round_frame) => {
    //                self.state
    //                    .replay(self.round_states.get(i), &self.round_states[i]);
    //                self.state().update();
    //                continue;
    //            }
    //            None => {
    //                break;
    //            }
    //        };
    //    }
    //}

    fn start_capture(&mut self) {
        loop {
            // Determine if we should sleep
            self.wait_to_proceed();
            self.state().update();

            let current_round = self.state().round();
            let round_frame_count = self.state().round_frame_count();
            let round_frame_count_previous = self.state().round_frame_count_previous();

            // If we're in the same frame, we'll want to wait until the next
            if round_frame_count == round_frame_count_previous {
                continue;
            }

            // Only update the batch once we have a new frame to advance
            self.round_states.push(*self.state().clone());

            // Save match data when the round count changes
            if round_frame_count < round_frame_count_previous {
                let round_result = serde_json::to_string(&self.round_states).unwrap();
                let file_name = format!(
                    "C:/Users/gsala/Documents/Tekken7Replays/{}_vs_{}_{}_{}.json",
                    self.state().character(globals::Player::One).to_string(),
                    self.state().character(globals::Player::Two).to_string(),
                    current_round.into(),
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
