use std::env;
use std::fs::*;
use std::io::Write;
use std::thread;

use read_process_memory::Pid;
use serde_json;
use sysinfo::{ProcessExt, SystemExt};

pub mod game_state;
pub mod globals;
pub mod input;
pub mod player;
pub mod position;
pub mod round;

fn main() {
    let args: Vec<_> = env::args().collect();

    let is_replay_mode = args[1] == "true";
    let mut reverse_sides = false;
    let mut replay_file = None;

    if is_replay_mode {
        replay_file = Some(serde_json::from_str::<Vec<game_state::ReplayFile>>(&read_to_string(args[2].clone()).unwrap()).unwrap());
        println!("Running in replay mode...");
    } 
    else {
        println!("Running in capture mode...");
    }

    if args.len() > 3 {
        reverse_sides = args[3] == "reverse";
    }

    let mut system = sysinfo::System::new();
    let mut tekken_pids = Vec::new();

    // Get all processes
    system.refresh_all();

    for (pid, proc_) in system.get_process_list() {
        if proc_.name() == "TekkenGame-Win64-Shipping.exe" {
            tekken_pids.push(pid);
        }
    }

    let tekken_pid = tekken_pids[0];

    let mut state = game_state::GameState::new(*tekken_pid as Pid);
    let mut round_batch = Vec::with_capacity(6000);

    loop {
        thread::sleep(std::time::Duration::from_secs_f64(1.0 / 120.0));
        // If false, then just record the match
        if replay_file.is_some() && state.round_frame_count().is_some() {
            state.update(Some(&replay_file.as_ref().unwrap()[state.round_frame_count().unwrap() as usize]), reverse_sides);
        }
        else {
            state.update(None, reverse_sides);
        }

        let current_round = state.round();
        let round_frame_count = state.round_frame_count();
        let round_frame_count_previous = state.round_frame_count_previous();

        //if current_round.is_none() {
        //    continue;
        //}

        if round_frame_count == round_frame_count_previous {
            continue;
        }

        // Only update the batch once we have a new frame to advance
        round_batch.push(state.clone());

        if round_frame_count < round_frame_count_previous && replay_file.is_none() {
            // Save match data when the round count changes
            let round_result = serde_json::to_string(&round_batch).unwrap();
            let file_name = format!(
                "C:/Users/gsala/Documents/Tekken7Replays/{}_{}_{}_{}.json", 
                state.character_id(globals::Player::One).map(|o| o.to_string()).unwrap_or("none".to_owned()), 
                state.character_id(globals::Player::Two).map(|o| o.to_string()).unwrap_or("none".to_owned()), 
                current_round.unwrap_or(0),
                uuid::Uuid::new_v4());

            println!("Creating file: {}", file_name);

            let mut file = File::create(file_name)
                .expect("Failed to create file");

            file.write(&round_result.as_bytes()).expect("Failed to write buffer to file");
            file.flush().expect("Failed to flush file");

            round_batch.clear();
            
            //C:/Users/gsala/Documents/Tekken7Replays/b0e5c34c-be0d-474a-b70f-2b56e934b920.json
        }
    }
}
