pub mod errors;
pub mod models;
pub mod wrappers;

use std::io::Cursor;

use byteorder::{ByteOrder, LittleEndian};
use read_process_memory::{ProcessHandle, copy_address};

use crate::globals::{MemoryAddress, Player};
pub use errors::MemoryReadErrors;
pub use wrappers::CursorWrapper;
pub use crate::globals::*;

pub trait MemoryModel {
    fn new(handle: ProcessHandle) -> Self;

    fn read<E, T>(&self, address: usize, length: usize) -> Result<T, Box<dyn std::error::Error>>
    where
        E: ByteOrder,
        T: CursorWrapper,
    {
        let data = copy_address(address, length, self.handle())
            .expect(&format!("Failed to read memory address: 0x{:X}", address));

        T::read::<E>(Cursor::new(data)).map(|v| v.into())
    }

    fn read_player<E, T>(
        &self,
        address: usize,
        player: Player,
        length: usize
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        E: ByteOrder,
        T: CursorWrapper,
    {
        let game_address = MemoryAddress::GameAddress as usize + self.player_base_address(player) + address;
        self.read::<E, T>(game_address, length)
    }

    fn inputted_attack(&self, player: Player) -> Result<u16, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(
            self.input_attack_address(player), 
            player, 
            4)
    }

    fn inputted_direction(&self, player: Player) -> Result<u16, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(
            self.input_direction_address(player), 
            player, 
            4)
    }

    fn damage_received(&self, player: Player) -> Result<u32, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u32>(
            self.player_health_address(player), 
            player, 
            4)
    }

    fn character(&self, player: Player) -> Result<Character, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(
                self.player_character_id_address(player), 
                player, 4)
            .map(|c| num::FromPrimitive::from_u16(c).unwrap_or(Character::NotSelected))
    }

    fn xyz(&self, player: Player) -> Result<(f32, f32, f32), Box<dyn std::error::Error>> {
        let x = self.read_player::<LittleEndian, f32>(
            self.player_coordinate_x(player),
            player,
            4,
        );
        let y = self.read_player::<LittleEndian, f32>(
            self.player_coordinate_y(player),
            player,
            4,
        );
        let z = self.read_player::<LittleEndian, f32>(
            self.player_coordinate_z(player),
            player,
            4,
        );

        Ok((x?, y?, z?))
    }

    fn coordinates(&self, player: Player) -> Result<(f32, f32, f32), Box<dyn std::error::Error>> {
        self.xyz(player)
    }

    fn facing(&self, player: Player) -> Result<Facing, Box<dyn std::error::Error>> {
        let facing_address = match player {
            Player::One => {
                MemoryAddress::GameAddress as usize + self.player_facing_address(player)
            }
            Player::Two => {
                MemoryAddress::GameAddress as usize + self.player_facing_address(player)
            }
        };

        let facing = self.read::<LittleEndian, u8>(facing_address, 4)?;

        match facing {
            0 => Ok(Facing::Left),
            1 => Ok(Facing::Right),
            _ => Err(Box::new(MemoryReadErrors::FacingTheVoid(
                player,
                errors::MemoryReadDiagnostics::new(
                    facing_address,
                    "DefaultMemoryModel::facing(&self)".into(),
                ),
            ))),
        }
    }

    fn ground_state(
        &self,
        _player: Player,
    ) -> Result<Option<GroundState>, Box<dyn std::error::Error>> {
        todo!();
    }
    
    fn round_frame(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let address =
            MemoryAddress::GameAddress as usize + self.round_timer_address();
        self.read::<LittleEndian, u32>(address, 4)
            .map(|v| v.into())
    }

    fn round(&self) -> Result<u8, Box<dyn std::error::Error>> {
        let address = MemoryAddress::GameAddress as usize + self.round_count_address();
        self.read::<LittleEndian, u8>(address, 4)
    }

    fn player_base_address(&self, player: Player)           -> usize;
    fn input_attack_address(&self, player: Player)          -> usize;
    fn input_direction_address(&self, player: Player)       -> usize;
    fn player_health_address(&self, player: Player)         -> usize;
    fn player_character_id_address(&self, player: Player)   -> usize;
    fn player_coordinate_x(&self, player: Player)           -> usize;
    fn player_coordinate_y(&self, player: Player)           -> usize;
    fn player_coordinate_z(&self, player: Player)           -> usize;
    fn player_facing_address(&self, player: Player)         -> usize;
    fn round_count_address(&self)                           -> usize;
    fn round_timer_address(&self)                           -> usize;
    /*
    fn character(&self, player: Player)         -> Result<crate::globals::Character, Box<dyn std::error::Error>>;
    fn damage_received(&self, player: Player)   -> Result<u32, Box<dyn std::error::Error>>;
    fn facing(&self, player: Player)            -> Result<crate::globals::Facing, Box<dyn std::error::Error>>;
    fn ground_state(&self, player: Player)      -> Result<Option<crate::globals::GroundState>, Box<dyn std::error::Error>>;
    fn xyz(&self, player: Player)               -> Result<(f32, f32, f32), Box<dyn std::error::Error>>;
    fn coordinates(&self, player: Player)       -> Result<(f32, f32, f32), Box<dyn std::error::Error>>;
    
    fn input_attack(&self, player: Player)      -> usize;
    fn input_direction(&self, player: Player)   -> usize;

    fn round_frame(&self)                       -> Result<u64, Box<dyn std::error::Error>>;
    fn round(&self)                             -> Result<u8, Box<dyn std::error::Error>>;
    */
    fn handle(&self) -> &ProcessHandle;

    /*
    fn replay(&self, previous_frame_state: Option<&GameState<M>>, frame_state: &GameState<M>) {
        if self.round_frame == self.round_frame_previous {
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
    */
}
