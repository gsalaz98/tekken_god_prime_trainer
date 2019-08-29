use std::fs::*;
use std::io;
use std::io::Write;
use std::thread;

use byteorder::{BigEndian, ReadBytesExt, LittleEndian, WriteBytesExt};
use read_process_memory::{TryIntoProcessHandle, Pid, CopyAddress, copy_address};
use serde_json;
use sysinfo::{ProcessExt, SystemExt};

pub mod game_state;
pub mod globals;
pub mod position;
pub mod round;

fn main() {
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
    let mut previous_frame_round = 1;

    loop {
        thread::sleep(std::time::Duration::from_secs_f64(1.0 / 60.0));
        state.update();

        round_batch.push(state.clone());

        let current_round = state.round();
        let round_frame_count = state.round_frame_count();
        let round_frame_count_previous = state.round_frame_count_previous();

        if current_round.is_none() {
            println!("Current round is None");
            continue;
        }

        if previous_frame_round != current_round.unwrap() {
            let round_result = serde_json::to_string(&round_batch).unwrap();
            let file_name = format!("C:/Users/gsala/Documents/Tekken7Replays/{}.json", uuid::Uuid::new_v4());

            println!("Creating file: {}", file_name);

            let mut file = File::create(file_name)
                .expect("Failed to create file");

            file.write(&round_result.as_bytes()).expect("Failed to write buffer to file");
            file.flush().expect("Failed to flush file");
        }

        previous_frame_round = current_round.unwrap();
    }
}
