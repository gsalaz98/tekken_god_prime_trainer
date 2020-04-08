use read_process_memory::*;
use serde::{Serialize, Deserialize};

use crate::globals;
use crate::input;
use crate::player;
use crate::position;
use crate::round;

#[derive(Clone, Deserialize, Serialize)]
pub struct GameState {
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

impl GameState {
    pub fn new() -> Self {
        Self {
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
    fn set_round_frame_count(&mut self, pid: &ProcessHandle) {
        match round::get_round_frame_count(pid) {
            Ok(frames) => self.round_frame_count = Some(frames),
            Err(e) => {
                self.round_frame_count = None;
                println!("Error reading match round timer frames: {:?}", e);
            }
        };
    }

    fn set_round(&mut self, pid: &ProcessHandle) {
        match round::get_round(pid) {
            Ok(round) => self.round = Some(round),
            Err(e) => {
                self.round = None;
                println!("Error reading match round count: {:?}", e);
            }
        };
    }

    pub fn update(&mut self, pid: &ProcessHandle) {
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
        self.set_round_frame_count(&pid);
        self.set_round(&pid);

        if self.round_frame_count == self.round_frame_count_previous {
            return;
        }

        let p1_coords = position::p1_xyz(pid);
        let p2_coords = position::p2_xyz(pid);

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

        self.p1_input_attack = input::get_inputted_attack(pid, globals::MemoryAddresses::PlayerOneBaseAddress).ok();
        self.p1_input_direction = input::get_inputted_direction(pid, globals::MemoryAddresses::PlayerOneBaseAddress).ok();
        self.p1_damage_received = player::get_damage_received(pid, globals::MemoryAddresses::PlayerOneBaseAddress).ok();
        self.p1_facing = position::facing(pid, globals::Player::One).ok();
        self.p1_char_id = player::get_player_char_id(pid, globals::MemoryAddresses::PlayerOneBaseAddress).ok();
        
        self.p2_input_attack = input::get_inputted_attack(pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        self.p2_input_direction = input::get_inputted_direction(pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        self.p2_damage_received = player::get_damage_received(pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        self.p2_facing = position::facing(pid, globals::Player::Two).ok();
        self.p2_char_id = player::get_player_char_id(pid, globals::MemoryAddresses::PlayerTwoBaseAddress).ok();
        
        println!("Frame: {}\tP1: {}, {} \t P2: {}, {}", self.round_frame_count.unwrap(), globals::InputButton::from(self.p1_input_attack.unwrap() as usize).to_str(), globals::InputDirection::from(self.p1_input_direction.unwrap() as usize).to_str(), globals::InputButton::from(self.p2_input_attack.unwrap() as usize).to_str(), globals::InputDirection::from(self.p2_input_direction.unwrap() as usize).to_str());
    }

    pub fn replay(&self, previous_frame_state: Option<&GameState>, frame_state: &GameState) {
        if self.round_frame_count == self.round_frame_count_previous {
            return;
        }

        let p1_input_attack = frame_state.p1_input_attack.expect("p1 input attack");
        let p1_input_direction = frame_state.p1_input_direction.expect("p1 input direction");
        let p1_button = globals::InputButton::from(p1_input_attack as usize);
        let p1_direction = globals::InputDirection::from(p1_input_direction as usize);
        let p1_facing = match frame_state.p1_facing {
            Some(0) => globals::Player::One,
            Some(1) => globals::Player::Two,
            _ => panic!("Player one is facing the void")
        };

        let p2_input_attack = frame_state.p2_input_attack.expect("p2 input attack");
        let p2_input_direction = frame_state.p2_input_direction.expect("p2 input direction");
        let p2_button = globals::InputButton::from(p2_input_attack as usize);
        let p2_direction = globals::InputDirection::from(p2_input_direction as usize);
        let p2_facing = match frame_state.p1_facing {
            Some(0) => globals::Player::One,
            Some(1) => globals::Player::Two,
            _ => panic!("Player two is facing the void")
        };

        // TODO: Figure out how to determine what side the player spawns on in online matches
        p1_button.input_attack(
            globals::Player::One, 
            previous_frame_state.map(|prev| {
                globals::InputButton::from(prev.p1_input_attack.unwrap() as usize)
            })
            .clone()
        );

        p1_direction.input_direction(
            globals::Player::One, 
            p1_facing.clone(), 
            previous_frame_state.map(|prev| match prev.p1_facing {
                Some(0) => globals::Player::One,
                Some(1) => globals::Player::Two,
                _ => panic!("Player one is facing the void")
            }), 
            previous_frame_state.map(|prev| {
                globals::InputDirection::from(prev.p1_input_direction.unwrap() as usize)
            })
        );

        // TODO: Figure out how to determine what side the player spawns on in online matches
        p2_button.input_attack(
            globals::Player::Two, 
            previous_frame_state.map(|prev| {
                globals::InputButton::from(prev.p2_input_attack.unwrap() as usize)
            })
        );

        p2_direction.input_direction(
            globals::Player::Two, 
            p2_facing.clone(), 
            previous_frame_state.map(|prev| match prev.p2_facing {
                Some(0) => globals::Player::One,
                Some(1) => globals::Player::Two,
                _ => panic!("Player two is facing the void")
            }), 
            previous_frame_state.map(|prev| {
                globals::InputDirection::from(prev.p2_input_direction.unwrap() as usize)
            })
        );
    }
}

