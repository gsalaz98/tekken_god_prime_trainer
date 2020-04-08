use std::fs::File;
use std::io::prelude::*;
use std::thread;

use read_process_memory::{Pid, ProcessHandle, TryIntoProcessHandle};

use crate::game_state::GameState;
use crate::globals;
use crate::replay::player_match::MatchReplay;

/// 
pub struct LoopState {
    pid: ProcessHandle,
    round_frames: Vec<GameState>,
    state: GameState,
    replay_mode: bool
}

impl LoopState {
    pub fn new(process: ProcessHandle, replay: Option<MatchReplay>) -> Self {
        Self {
            // Process ID for the Tekken 7 game executable
            pid: process,
            /// Pre-allocate 3600 frames of data (1 minute)
            /// which is the max amount of frames that can exist 
            /// in a Tekken 7 match using official tournament rules
            /// (excludes round-victory inputs)
            round_frames: match replay {
                Some(frames) => frames.round_frames(),
                None => Vec::with_capacity(3600)
            },
            /// Current state of the game (e.g. data of the most recent frame received)
            state: GameState::new(),
            replay_mode: replay.is_some()
        }
    }

    /// Sleeps until the frame count has been updated
    fn wait_to_proceed(&self) {
        match self.state.round_frame_count_previous(), self.state.round_frame_count() {
            Some(previous_frame, current_frame) if previous_frame == current_frame => 
                let time_to_sleep_ms = 1.0f64 / 120.0f64;
                // Sleep every 1/120th of a second if we're still in the same frame so that we can save our processing power
                thread::sleep(std::time::Duration::from_secs_f64(time_to_sleep_ms));
            },
            None => ()
        };
    }

    pub fn start(&mut self) {
        match self.replay_mode {
            true => self.start_replay(),
            false => self.start_capture()
        };
    }

    fn start_replay(&mut self) {
        // Wait until we're in a game to start updating the frames
        while let None = self.state.round_frame_count() {
            self.wait_to_proceed();
            self.state.update(&self.pid);
        }

        for i in 0..self.round_frames.len() {
            // Determine if we should sleep. On the first frame
            // this is guaranteed to be skipped because the previous frame 
            // hasn't been set yet, which means we begin the replay the
            // moment we know we're in a game
            self.wait_to_proceed();

            match self.state.round_frame_count() {
                Some(round_frame) => {
                    self.state.replay(&self.round_frames.get(i), &self.round_frames[i]);
                    self.state.update(&self.pid);
                    continue;
                },
                None => break;
            };
        }
    }

    fn start_capture(&mut self) {
        loop {
            // Determine if we should sleep
            self.wait_to_proceed();
            self.state.update(&self.pid);

            let current_round = self.state.round();
            let round_frame_count = self.state.round_frame_count();
            let round_frame_count_previous = self.state.round_frame_count_previous();

            // If we're in the same frame, we'll want to wait until the next
            if round_frame_count == round_frame_count_previous {
                continue;
            }

            // Only update the batch once we have a new frame to advance
            self.round_frames.push(self.state.clone());
        
            // Save match data when the round count changes
            if round_frame_count < round_frame_count_previous {
                let round_result = serde_json::to_string(&self.round_frames).unwrap();
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

                self.round_frames.clear();
            }
        }
    }
}