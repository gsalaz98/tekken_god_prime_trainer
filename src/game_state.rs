use read_process_memory::*;
use serde::{Serialize, Deserialize};

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
    p1_facing: Option<u8>,

    p2_x: Option<f32>,
    p2_y: Option<f32>,
    p2_z: Option<f32>,
    p2_input_attack: Option<u16>,
    p2_input_direction: Option<u16>,
    p2_damage_received: Option<u32>,
    p2_facing: Option<u8>,

    last_update: Option<f64>,

    #[serde(skip)]
    p1_char_id: Option<u16>,
    #[serde(skip)]
    p2_char_id: Option<u16>,

    #[serde(skip)]
    p1_previous_button: Option<globals::InputButton>,
    #[serde(skip)]
    p1_previous_direction: Option<globals::InputDirection>,
    #[serde(skip)]
    p1_previous_facing: Option<globals::Player>,

    #[serde(skip)]
    p2_previous_button: Option<globals::InputButton>,
    #[serde(skip)]
    p2_previous_direction: Option<globals::InputDirection>,
    #[serde(skip)]
    p2_previous_facing: Option<globals::Player>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    round_frame_count: u32,
    round_frame_count_previous: u32,

    round: u8,
    
    p1_x: f32,
    p1_y: f32,
    p1_z: f32,
    p1_input_attack: u16,
    p1_input_direction: u16,
    p1_damage_received: u32,
    p1_facing: u8,

    p2_x: f32,
    p2_y: f32,
    p2_z: f32,
    p2_input_attack: u16,
    p2_input_direction: u16,
    p2_damage_received: u32,
    p2_facing: u8,

    last_update: f64,
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
            p1_facing: None,

            p2_x: None,
            p2_y: None,
            p2_z: None,
            p2_input_attack: None,
            p2_input_direction: None,
            p2_damage_received: None,
            p2_facing: None,

            p1_char_id: None,
            p2_char_id: None,

            last_update: None,
            p1_previous_button: None,
            p1_previous_direction: None,
            p1_previous_facing: None,

            p2_previous_button: None,
            p2_previous_direction: None,
            p2_previous_facing: None,
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

    pub fn character_id(&self, player: globals::Player) -> Option<u16> {
        match player {
            globals::Player::One => self.p1_char_id,
            globals::Player::Two => self.p2_char_id
        }
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

    pub fn update(&mut self, replay_file_frame: Option<&ReplayFile>, reverse_sides: bool) {
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
        self.p1_facing = position::facing(&self.pid, globals::Player::One).ok();
        self.p1_char_id = player::get_player_char_id(&self.pid, globals::MemoryAddresses::PlayerOneBaseAddress).ok();
        
        self.p2_input_attack = input::get_inputted_attack(&self.pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        self.p2_input_direction = input::get_inputted_direction(&self.pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        self.p2_damage_received = player::get_damage_received(&self.pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        self.p2_facing = position::facing(&self.pid, globals::Player::Two).ok();
        self.p2_char_id = player::get_player_char_id(&self.pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        

        if replay_file_frame.is_some() {
            let replay = replay_file_frame.unwrap();

            let mut p1_button = globals::InputButton::from(replay.p1_input_attack as usize);
            let mut p1_direction = globals::InputDirection::from(replay.p1_input_direction as usize);
            let p1_facing = if replay.p1_facing == 0 { globals::Player::One } else { globals::Player::Two };

            let mut p2_button = globals::InputButton::from(replay.p2_input_attack as usize);
            let mut p2_direction = globals::InputDirection::from(replay.p2_input_direction as usize);
            let p2_facing = if replay.p2_facing == 0 { globals::Player::One } else { globals::Player::Two };

            if reverse_sides {
                std::mem::swap(&mut p1_button, &mut p2_button);
                std::mem::swap(&mut p1_direction, &mut p2_direction);
            }

            println!("Frame: {}\tP1: {}, {}, {}, {}", self.round_frame_count.unwrap(), p1_button.to_str(), p1_direction.to_str(), replay.p1_facing, p1_direction.to_input_key(&globals::Player::One, p1_facing.clone()));

            p1_button.input_attack(globals::Player::One, self.p1_previous_button.clone());
            p1_direction.input_direction(globals::Player::One, p1_facing.clone(), self.p1_previous_facing.clone(), self.p1_previous_direction.clone());
            p2_button.input_attack(globals::Player::Two, self.p2_previous_button.clone());
            p2_direction.input_direction(globals::Player::Two, p2_facing.clone(), self.p2_previous_facing.clone(), self.p2_previous_direction.clone());

            self.p1_previous_button = Some(p1_button);
            self.p1_previous_direction = Some(p1_direction);
            self.p1_previous_facing = Some(p1_facing);

            self.p2_previous_button = Some(p2_button);
            self.p2_previous_direction = Some(p2_direction);
            self.p2_previous_facing = Some(p2_facing);
        }
        else {
            println!("Frame: {}\tP1: {}, {} \t P2: {}, {}", self.round_frame_count.unwrap(), globals::InputButton::from(self.p1_input_attack.unwrap() as usize).to_str(), globals::InputDirection::from(self.p1_input_direction.unwrap() as usize).to_str(), globals::InputButton::from(self.p2_input_attack.unwrap() as usize).to_str(), globals::InputDirection::from(self.p2_input_direction.unwrap() as usize).to_str());
        }
    }
}

