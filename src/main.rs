use std::env;
use std::fs::*;
use std::thread;

use read_process_memory::Pid;
use serde_json;

pub mod event_loop;
pub mod game_state;
pub mod globals;
pub mod input;
pub mod player;
pub mod position;
pub mod round;
pub mod util;

use game_state::GameState;

/// Name of the executable to search for
pub const EXECUTABLE_NAME: &'static str = "TekkenGame-Win64-Shipping.exe";

fn main() {
    let args: Vec<_> = env::args().collect();

    let replay_mode = args[1] == "true";
    let mut reverse_sides = false;
    let replay_states = match replay_mode {
        true => {
            println!("Running in replay mode...");
            let replay_path = args[2].clone();
            let replay_contents = read_to_string(replay_path).unwrap();
            let replay_state = serde_json::from_str::<Vec<GameState>>(&replay_contents).unwrap();

            Some(replay_state)
        },
        false => {
            println!("Running in capture mode...");
            None
        }
    };

    if args.len() > 3 {
        reverse_sides = args[3] == "reverse";
    }

    let mut process_id = util::get_pid();

    // Try every 10 seconds to get the Tekken PID
    while process_id.is_err() {
        println!("Error encountered: {} - Retrying in 10s", process_id.unwrap_err());
        thread::sleep(std::time::Duration::from_secs(10));

        process_id = util::get_pid();
    }


    let mut loop_state = match replay_states {
        Some(states) => event_loop::LoopState::from_replay(process_id.unwrap() as Pid, states),
        None => event_loop::LoopState::new(process_id.unwrap() as Pid)
    };

    // Begin capturing or replaying
    loop_state.start();
}
