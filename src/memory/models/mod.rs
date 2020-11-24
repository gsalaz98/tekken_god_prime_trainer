pub mod season_three;

use std::io::Cursor;

use byteorder::{ByteOrder, LittleEndian};
use num_traits::FromPrimitive;
use read_process_memory::*;

use super::errors::MemoryReadDiagnostics;
use crate::globals::{Character, Facing, GroundState, MemoryAddress, Player};
use crate::memory::models::season_three::V3Dot33;
use crate::memory::wrappers::CursorWrapper;
use crate::memory::MemoryModel;
use crate::memory::MemoryReadErrors;


    /*
    fn damage_received(&self, player: Player) -> Result<Self::Damage, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u32>(MemoryAddress::PlayerHealthOffset, player, 4)
    }

    fn character(&self, player: Player) -> Result<Character, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(MemoryAddress::PlayerCharacterID, player, 4)
            .map(|c| FromPrimitive::from_u16(c).unwrap_or(Character::NotSelected))
    }

    fn xyz(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>> {
        let x = self.read_player::<LittleEndian, f32>(
            MemoryAddress::PlayerCoordinateOffsetX,
            player,
            4,
        );
        let y = self.read_player::<LittleEndian, f32>(
            MemoryAddress::PlayerCoordinateOffsetY,
            player,
            4,
        );
        let z = self.read_player::<LittleEndian, f32>(
            MemoryAddress::PlayerCoordinateOffsetZ,
            player,
            4,
        );

        Ok((x?, y?, z?))
    }

    fn coordinates(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>> {
        self.xyz(player)
    }

    fn facing(&self, player: Player) -> Result<Facing, Box<dyn std::error::Error>> {
        let facing_address = match player {
            Player::One => {
                MemoryAddress::GameAddress as usize + MemoryAddress::PlayerOneFacing as usize
            }
            Player::Two => {
                MemoryAddress::GameAddress as usize + MemoryAddress::PlayerTwoFacing as usize
            }
        };

        let facing = self.read::<LittleEndian, u8>(facing_address, 4)?;

        match facing {
            0 => Ok(Facing::Left),
            1 => Ok(Facing::Right),
            _ => Err(Box::new(MemoryReadErrors::FacingTheVoid(
                player,
                MemoryReadDiagnostics::new(
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
            MemoryAddress::GameAddress as usize + MemoryAddress::RoundTimerInFrames as usize;
        self.read::<LittleEndian, Self::FrameCount>(address, 4)
            .map(|v| v.into())
    }

    fn round(&self) -> Result<u8, Box<dyn std::error::Error>> {
        let address = MemoryAddress::GameAddress as usize + MemoryAddress::RoundCount as usize;
        self.read::<LittleEndian, Self::RoundCount>(address, 4)
    }
}
*/
