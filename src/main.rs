//! # `god_prime_trainer` - Training bot for Tekken 7
//!
//! This application is used to capture data for Tekken 7 matches.
//! That data will then be used to analyze the game state and provide
//! suggestions for optimal moves, similar to stockfish in chess.
 
mod errors;
pub mod globals;
pub mod memory;
pub mod states;
pub mod util;

use std::env;
use std::thread;

use clap::{self, App, Arg};
use read_process_memory::*;

use crate::memory::models::season_three::V3Dot33;
use crate::states::loop_state::LoopState;

/// Name of the executable to search for
pub const EXECUTABLE_NAME: &'static str = "TekkenGame-Win64-Shipping.exe";

fn main() {
    let args = App::new("TEKKEN God Prime Trainer")
        .version(&clap::crate_version!()[..])
        .arg(
            Arg::with_name("reverse")
                .short("s")
                .long("reverse")
                .value_name("REVERSE")
                .help("Reverses the player's side")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out")
                .value_name("out")
                .help("Sets the output directory of the data")
                .takes_value(true)
                .conflicts_with_all(&["replay", "replay_path", "reverse"])
                .required(true),
        )
        .get_matches();

    println!("Starting in capture mode...");
    let mut process_id = util::pid();

    // Try every 10 seconds to get the Tekken PID
    while let Err(pid) = process_id {
        println!(
            "Failed to acquire process ID: {:?} - Retrying in 10s...",
            pid
        );
        thread::sleep(std::time::Duration::from_secs(10));
        process_id = util::pid();
    }

    let process_handle = process_id
        .unwrap()
        .try_into_process_handle()
        .expect("Failed to create Tekken 7 process handle. Please restart and try again.");

    let mut loop_state = LoopState::<V3Dot33>::new(); //, match_replay);

    // Begin capturing
    loop_state.start(process_handle);
}
