use read_process_memory::*;
use serde::{Deserialize, Serialize};

use crate::globals::{self, Character, Player};
use crate::memory::{InputMemory, MemoryModel, RoundMemory};
use crate::states::{
    builders::GameStateBuilder,
    player_state::{DefaultPlayerInfo, DefaultPlayerState},
};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct GameState<M: MemoryModel> {
    round: M::RoundCount,
    round_frame_count: M::FrameCount,

    // TODO: Make this a generic trait definition that can be Serialized + Deserialized
    player_info: (DefaultPlayerInfo, DefaultPlayerInfo),
    player_state: (DefaultPlayerState, DefaultPlayerState),

    #[serde(skip)]
    memory: Option<M>,
}

impl<M: MemoryModel> GameState<M> {
    pub fn new(builder: GameStateBuilder<M>) -> Self {
        builder.build()
    }

    pub fn player(&self, player: Player) -> DefaultPlayerState {
        match player {
            Player::One => self.player_state.0,
            Player::Two => self.player_state.1,
        }
    }

    pub fn player_info(&self, player: Player) -> DefaultPlayerInfo {
        match player {
            Player::One => self.player_info.0,
            Player::Two => self.player_info.1,
        }
    }

    pub fn round(&self) -> M::RoundCount {
        self.round
    }

    pub fn round_frame_count(&self) -> M::FrameCount {
        self.round_frame_count
    }

    pub fn round_frame_count_previous(&self) -> M::FrameCount {
        self.round_frame_count_previous
    }

    pub fn character(&self, player: globals::Player) -> Character {
        match player {
            globals::Player::One => self.player(Player::One).character,
            globals::Player::Two => self.player(Player::Two).character,
        }
    }

    /// Sets the frame count for the current frame
    fn update_round_frame_count(&mut self) {
        match self.memory.round_frame() {
            Ok(frames) => self.round_frame_count = Some(frames.into()),
            Err(e) => {
                self.round_frame_count = None;
                println!("Error reading match round timer frames: {:?}", e);
            }
        };
    }

    fn update_round(&mut self) {
        match self.memory.round() {
            Ok(round) => self.round = Some(round.into()),
            Err(e) => {
                self.round = None;
                println!("Error reading match round count: {:?}", e);
            }
        };
    }

    pub fn update_player_state(&self, player: Player) {
        match player {
            Player::One => {
                let coordinates = self.memory.coordinates(Player::One);

                self.p1_x = coordinates.x().ok();
                self.p1_y = coordinates.y().ok();
                self.p1_z = coordinates.z().ok();

                self.p1_input_attack = self.memory.input_attack(player).ok();
                self.p1_input_direction = self.memory.input_direction(player).ok();

                self.p1_damage_received = self.memory.player_damage_received(player).ok();
                self.p1_facing = self.memory.player_facing(player).ok();
                self.p1_char_id = self.memory.player_character_id(player).ok();
            }
            Player::Two => {
                let (x, y, z) = self.memory.player_coordinates(player);

                self.p2_x = x.ok();
                self.p2_y = y.ok();
                self.p2_z = z.ok();

                self.p2_input_attack = self.memory.input_attack(player).ok();
                self.p2_input_direction = self.memory.input_direction(player).ok();

                self.p2_damage_received = self.memory.player_damage_received(player).ok();
                self.p2_facing = self.memory.player_facing(player).ok();
                self.p2_char_id = self.memory.player_character_id(player).ok();
            }
        };
    }

    pub fn update(&mut self) {
        self.round_frame_count_previous = self.round_frame_count;

        // Make sure we update the last update field as the first thing we do
        // since `update` will ALWAYS result in a mutation
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(time) => self.last_update = Some(time.as_millis() as f64 * 0.001),
            Err(_) => println!("Failed to get system time. Defaulting to previous value"),
        };

        // Update the previous frame count here so that we don't
        // screw up ourselves somehow. Will always be None at initialization time
        if self.round_frame_count_previous.is_none() {
            self.round_frame_count_previous = self.round_frame_count;
        }

        // Set the timer to None if we encounter an error setting the frame count
        self.update_round_frame_count();
        self.update_round();

        if self.round_frame_count == self.round_frame_count_previous {
            return;
        }

        self.update_player_state(Player::One);
        self.update_player_state(Player::Two);

        println!(
            "Frame: {}\tP1: {}, {} \t P2: {}, {}",
            self.round_frame_count.unwrap(),
            globals::InputButton::from(self.p1_input_attack.unwrap() as usize).to_str(),
            globals::InputDirection::from(self.p1_input_direction.unwrap() as usize).to_str(),
            globals::InputButton::from(self.p2_input_attack.unwrap() as usize).to_str(),
            globals::InputDirection::from(self.p2_input_direction.unwrap() as usize).to_str()
        );
    }

    pub fn replay(&self, previous_frame_state: Option<&GameState<M>>, frame_state: &GameState<M>) {
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
            _ => panic!("Player one is facing the void"),
        };

        let p2_input_attack = frame_state.p2_input_attack.expect("p2 input attack");
        let p2_input_direction = frame_state.p2_input_direction.expect("p2 input direction");
        let p2_button = globals::InputButton::from(p2_input_attack as usize);
        let p2_direction = globals::InputDirection::from(p2_input_direction as usize);
        let p2_facing = match frame_state.p1_facing {
            Some(0) => globals::Player::One,
            Some(1) => globals::Player::Two,
            _ => panic!("Player two is facing the void"),
        };

        // TODO: Figure out how to determine what side the player spawns on in online matches
        p1_button.input_attack(
            globals::Player::One,
            previous_frame_state
                .map(|prev| globals::InputButton::from(prev.p1_input_attack.unwrap() as usize))
                .clone(),
        );

        p1_direction.input_direction(
            globals::Player::One,
            p1_facing.clone(),
            previous_frame_state.map(|prev| match prev.p1_facing {
                Some(0) => globals::Player::One,
                Some(1) => globals::Player::Two,
                _ => panic!("Player one is facing the void"),
            }),
            previous_frame_state.map(|prev| {
                globals::InputDirection::from(prev.p1_input_direction.unwrap() as usize)
            }),
        );

        // TODO: Figure out how to determine what side the player spawns on in online matches
        p2_button.input_attack(
            globals::Player::Two,
            previous_frame_state
                .map(|prev| globals::InputButton::from(prev.p2_input_attack.unwrap() as usize)),
        );

        p2_direction.input_direction(
            globals::Player::Two,
            p2_facing.clone(),
            previous_frame_state.map(|prev| match prev.p2_facing {
                Some(0) => globals::Player::One,
                Some(1) => globals::Player::Two,
                _ => panic!("Player two is facing the void"),
            }),
            previous_frame_state.map(|prev| {
                globals::InputDirection::from(prev.p2_input_direction.unwrap() as usize)
            }),
        );
    }
}
