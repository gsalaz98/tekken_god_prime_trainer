//! # `god_prime_trainer` - Training bot for Tekken 7
//! 
//! This bot provides additional training functionality that the game
//! fails to provide itself. In addition, it is capable of recording
//! matches for frame-by-frame analysis of the game state.

mod errors;
pub mod event_loop;
pub mod game_state;
pub mod player_state;
pub mod globals;
pub mod memory;
pub mod replay;
pub mod util;

use std::env;
use std::fs::*;
use std::thread;

use clap::{self, Arg, App};
use read_process_memory::*;
use serde_json;

use event_loop::LoopState;
use replay::player_match::MatchReplay;

/// Name of the executable to search for
pub const EXECUTABLE_NAME: &'static str = "TekkenGame-Win64-Shipping.exe";

fn main() {
    let args = App::new("God Prime Trainer")
        .version(&clap::crate_version!()[..])
        .arg(Arg::with_name("replay")
            .short("r")
            .long("replay")
            .value_name("REPLAY_MODE")
            .help("Starts the bot in replay mode")
            .required(false)
            .takes_value(false))
        .arg(Arg::with_name("replay_path")
            .short("i")
            .long("replay_path")
            .value_name("REPLAY_PATH")
            .help("Sets the path to the replay file")
            .takes_value(true)
            .requires("replay")
            .required(true))
        .arg(Arg::with_name("reverse")
            .short("s")
            .long("reverse")
            .value_name("REVERSE")
            .help("Reverses the player's side")
            .required(false)
            .takes_value(false))
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .value_name("out")
            .help("Sets the output directory when running in capture mode")
            .takes_value(true)
            .conflicts_with_all(&["replay", "replay_path", "reverse"])
            .required(true))
        .get_matches();

    let reverse_sides = args.is_present("reverse");
    let match_replay = match args.is_present("replay") {
        true => {
            println!("Starting in replay mode...");

            let replay_path = args.value_of("replay_path")
                .expect("Replay mode requires input file argument to be set [--replay_path]");

            let replay_contents = read_to_string(&replay_path).unwrap();
            let replay_state = serde_json::from_str::<MatchReplay>(&replay_contents).expect("Bad replay file");

            Some(replay_state)
        },
        false => {
            println!("Starting in capture mode...");
            None
        }
    };

    let mut process_id = util::get_pid();

    // Try every 10 seconds to get the Tekken PID
    while let Err(pid) = process_id {
        println!("Failed to acquire process ID: {:?} - Retrying in 10s...", pid);
        thread::sleep(std::time::Duration::from_secs(10));
        process_id = util::get_pid();
    }

    let process_handle = process_id.unwrap()
        .try_into_process_handle()
        .expect("Failed to create Tekken 7 process handle");

    //let mut looper = LoopState::new(&process_id.unwrap().try_into_process_handle(), replay_states);
    let mut loop_state = LoopState::new(process_handle, match_replay);

    // Begin capturing or replaying
    loop_state.start();
}
