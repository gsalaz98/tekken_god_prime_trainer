use read_process_memory::*;
use serde::Serialize;

use crate::globals;
use crate::input;
use crate::player;
use crate::position;
use crate::round;

#[derive(Clone, Serialize)]
pub struct GameState {
    #[serde(skip)]
    pid: *mut std::ffi::c_void,

    round_frame_count: Option<u32>,
    round_frame_count_previous: Option<u32>,

    round: Option<u8>,
    
    p1_x: Option<f32>,
    p1_y: Option<f32>,
    p1_z: Option<f32>,
    p1_input_attack: Option<u16>,
    p1_input_direction: Option<u16>,
    p1_damage_received: Option<u32>,

    p2_x: Option<f32>,
    p2_y: Option<f32>,
    p2_z: Option<f32>,
    p2_input_attack: Option<u16>,
    p2_input_direction: Option<u16>,
    p2_damage_received: Option<u32>,

    last_update: Option<f64>,
}

impl GameState {
    pub fn new(pid: Pid) -> Self {
        Self {
            pid: pid.try_into_process_handle().expect("Failed to get process handle"),

            round_frame_count: None,
            round_frame_count_previous: None,

            round: None,

            p1_x: None,
            p1_y: None,
            p1_z: None,
            p1_input_attack: None,
            p1_input_direction: None,
            p1_damage_received: None,

            p2_x: None,
            p2_y: None,
            p2_z: None,
            p2_input_attack: None,
            p2_input_direction: None,
            p2_damage_received: None,

            last_update: None
        }
    }

    pub fn round(&self) -> Option<u8> {
        self.round
    }
    
    pub fn round_frame_count(&self) -> Option<u32> {
        self.round_frame_count
    }

    pub fn round_frame_count_previous(&self) -> Option<u32> {
        self.round_frame_count_previous
    }

    /// Sets the frame count for the current frame
    fn set_round_frame_count(&mut self) {
        match round::get_round_frame_count(&self.pid) {
            Ok(frames) => self.round_frame_count = Some(frames),
            Err(e) => {
                self.round_frame_count = None;
                println!("Error reading match round timer frames: {:?}", e);
            }
        };
    }

    fn set_round(&mut self) {
        match round::get_round(&self.pid) {
            Ok(round) => self.round = Some(round),
            Err(e) => {
                self.round = None;
                println!("Error reading match round count: {:?}", e);
            }
        };
    }

    pub fn update(&mut self) {
        self.round_frame_count_previous = self.round_frame_count;

        // Make sure we update the last update field as the first thing we do
        // since `update` will ALWAYS result in a mutation
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(time) => self.last_update = Some(time.as_millis() as f64 * 0.001),
            Err(_) => println!("Failed to get system time. Defaulting to previous value")
        };

        // Update the previous frame count here so that we don't
        // screw up ourselves somehow. Will always be None at initialization time
        if self.round_frame_count_previous.is_none() {
            self.round_frame_count_previous = self.round_frame_count;
        }

        // Set the timer to None if we encounter an error setting the frame count
        self.set_round_frame_count();
        self.set_round();

        if self.round_frame_count == self.round_frame_count_previous {
            return;
        }

        let p1_coords = position::p1_xyz(&self.pid);
        let p2_coords = position::p2_xyz(&self.pid);

        match p1_coords {
            Ok((x, y, z)) => {
                self.p1_x = Some(x);
                self.p1_y = Some(y);
                self.p1_z = Some(z);
            },
            Err(_) => {
                self.p1_x = None;
                self.p1_y = None;
                self.p1_z = None;
            }
        };

        match p2_coords {
            Ok((x, y, z)) => {
                self.p2_x = Some(x);
                self.p2_y = Some(y);
                self.p2_z = Some(z);
            },
            Err(_) => {
                self.p2_x = None;
                self.p2_y = None;
                self.p2_z = None;
            }
        };

        self.p1_input_attack = input::get_inputted_attack(&self.pid, globals::MemoryAddresses::PlayerOneBaseAddress).ok();
        self.p1_input_direction = input::get_inputted_direction(&self.pid, globals::MemoryAddresses::PlayerOneBaseAddress).ok();
        self.p1_damage_received = player::get_damage_received(&self.pid, globals::MemoryAddresses::PlayerOneBaseAddress).ok();

        self.p2_input_attack = input::get_inputted_attack(&self.pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        self.p2_input_direction = input::get_inputted_direction(&self.pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        self.p2_damage_received = player::get_damage_received(&self.pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
    }
}