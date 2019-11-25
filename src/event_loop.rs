use std::fs::File;
use std::io::prelude::*;
use std::thread;

use read_process_memory::{Pid, ProcessHandle, TryIntoProcessHandle};
use crate::game_state::GameState;
use crate::globals;


pub struct LoopState {
    pid: ProcessHandle,
    round_states: Vec<GameState>,
    state: GameState,
    replay_mode: bool
}

impl LoopState {
    fn maybe_sleep(&self) {
        match self.state.round_frame_count_previous() {
            Some(previous_frame) => match self.state.round_frame_count() {
                Some(current_frame) => {
                    if previous_frame == current_frame {
                        let time_to_sleep_ms = 1.0f64 / 120.0f64;
                        // Sleep every 1/120th of a second if we're still in the same frame so that we can save our processing power
                        thread::sleep(std::time::Duration::from_secs_f64(time_to_sleep_ms));
                    }
                },
                None => ()
            },
            None => ()
        };
    }

    pub fn new(tekken_pid: Pid) -> Self {
        Self {
            // Process ID for the Tekken 7 game executable
            pid: tekken_pid.try_into_process_handle().unwrap(),
            // Pre-allocate 18000 frames (about 5 minutes of playing time)
            // which is the max amount of frames that a best of 5 round game of Tekken can have
            round_states: Vec::with_capacity(18000),
            // Current state of the game
            state: GameState::new(),
            // Use `from_replay` if you want to initialize a replay           
            replay_mode: false
        }
    }

    pub fn from_replay(tekken_pid: Pid, replay_states: Vec<GameState>) -> Self {
        Self {
            // Process ID for the Tekken 7 game executable
            pid: tekken_pid.try_into_process_handle().unwrap(),
            round_states: replay_states,
            state: GameState::new(),
            replay_mode: true
        }
    }

    pub fn start(&mut self) {
        match self.replay_mode {
            true => self.start_replay(),
            false => self.start_capture()
        };
    }

    fn start_replay(&mut self) {
        // Wait until we're in a game to start updating the frames
        while self.state.round_frame_count().is_none() {
            self.maybe_sleep();
            self.state.update(&self.pid);
        }

        for i in 0..self.round_states.len() {
            // Determine if we should sleep. On the first frame
            // this is guaranteed to be skipped because the previous frame 
            // hasn't been set yet, which means we begin the replay the
            // moment we know we're in a game
            self.maybe_sleep();

            if self.state.round_frame_count().is_some() {
                let previous_frame = match i {
                    0 => None,
                    _ => Some(&self.round_states[i - 1])
                };

                self.state.replay(previous_frame, &self.round_states[i]);
                self.state.update(&self.pid);
                continue;
            }
            
            break;
        }
    }

    fn start_capture(&mut self) {
        loop {
            // Determine if we should sleep
            self.maybe_sleep();
            self.state.update(&self.pid);

            let current_round = self.state.round();
            let round_frame_count = self.state.round_frame_count();
            let round_frame_count_previous = self.state.round_frame_count_previous();

            // If we're in the same frame, we'll want to wait until the next
            if round_frame_count == round_frame_count_previous {
                continue;
            }

            // Only update the batch once we have a new frame to advance
            self.round_states.push(self.state.clone());
        
            // Save match data when the round count changes
            if round_frame_count < round_frame_count_previous {
                let round_result = serde_json::to_string(&self.round_states).unwrap();
                let file_name = format!(
                    "C:/Users/gsala/Documents/Tekken7Replays/{}_{}_{}_{}.json", 
                    self.state.character_id(globals::Player::One).map(|o| o.to_string()).unwrap_or("none".to_owned()), 
                    self.state.character_id(globals::Player::Two).map(|o| o.to_string()).unwrap_or("none".to_owned()), 
                    current_round.unwrap_or(0),
                    uuid::Uuid::new_v4());

                println!("Creating file: {}", file_name);

                let mut file = File::create(file_name)
                    .expect("Failed to create file");

                file.write(&round_result.as_bytes()).expect("Failed to write buffer to file");
                file.flush().expect("Failed to flush file");

                self.round_states.clear();
            }
        }
    }
}