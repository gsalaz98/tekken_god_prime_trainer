use read_process_memory::*;
use serde::Serialize;

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

    p2_x: Option<f32>,
    p2_y: Option<f32>,
    p2_z: Option<f32>,

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

            p2_x: None,
            p2_y: None,
            p2_z: None,

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
        // Store the previous last update time just in case
        let _prev_last_update = self.last_update;

        // Make sure we update the last update field as the first thing we do
        // since `update` will ALWAYS result in a mutation
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(time) => self.last_update = Some(time.as_millis() as f64 * 0.001),
            Err(_) => println!("Failed to get system time. Defaulting to previous value")
        };

        // Update the previous frame count here so that we don't
        // screw up ourselves somehow
        if self.round_frame_count_previous.is_none() {
            self.round_frame_count_previous = self.round_frame_count;
        }

        // Set the timer to None if we encounter an error setting the frame count
        self.set_round_frame_count();
        self.set_round();

        if self.round_frame_count == self.round_frame_count_previous {
            println!("You are not in a match currently or are in between rounds.");
            return;
        }

        let p1_coords = position::p1_xyz(&self.pid);
        let p2_coords = position::p2_xyz(&self.pid);

        match p1_coords {
            Ok(coordinates) => {
                self.p1_x = Some(coordinates.0);
                self.p1_y = Some(coordinates.1);
                self.p1_z = Some(coordinates.2);
            },
            Err(_) => {
                self.p1_x = None;
                self.p1_y = None;
                self.p1_z = None;
            }
        };

        match p2_coords {
            Ok(coordinates) => {
                self.p2_x = Some(coordinates.0);
                self.p2_y = Some(coordinates.1);
                self.p2_z = Some(coordinates.2);
            },
            Err(_) => {
                self.p2_x = None;
                self.p2_y = None;
                self.p2_z = None;
            }
        };
    }
}