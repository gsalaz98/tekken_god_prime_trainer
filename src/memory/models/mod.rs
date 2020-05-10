use std::io::Cursor;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use read_process_memory::*;

use super::errors::MemoryReadDiagnostics;
use crate::globals::{Character, Facing, GroundState, MemoryAddress, Player};
use crate::memory::input::InputMemory;
use crate::memory::player::PlayerMemory;
use crate::memory::round::RoundMemory;
use crate::memory::wrappers::CursorWrapper;
use crate::memory::MemoryModel;
use crate::memory::MemoryReadErrors;

#[derive(Clone)]
pub struct DefaultMemoryModel {
    handle: ProcessHandle,
}

impl MemoryModel for DefaultMemoryModel {
    fn new(handle: ProcessHandle) -> Self {
        Self { handle }
    }

    fn read<E, T>(&self, address: usize, length: usize) -> Result<T, Box<dyn std::error::Error>>
    where
        E: ByteOrder,
        T: CursorWrapper,
    {
        let data = copy_address(address, length, &self.handle)
            .expect(&format!("Failed to read memory address: 0x{:X}", address));

        T::read::<E>(Cursor::new(data)).map(|v| v.into())
    }

    fn read_player<E, T>(
        &self,
        address: MemoryAddress,
        player: Player,
        length: usize,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        E: ByteOrder,
        T: CursorWrapper,
    {
        let game_address =
            MemoryAddress::GameAddress as usize + player.base_address() + address as usize;
        self.read::<E, T>(game_address, length)
    }

    fn handle(&self) -> &ProcessHandle {
        &self.handle
    }
}

impl InputMemory for DefaultMemoryModel {
    type GameInput = u16;
    fn inputted_attack(
        &self,
        player: Player,
    ) -> Result<Self::GameInput, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(MemoryAddress::InputAttack, player, 4)
    }

    fn inputted_direction(
        &self,
        player: Player,
    ) -> Result<Self::GameInput, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(MemoryAddress::InputDirection, player, 4)
    }
}

impl PlayerMemory for DefaultMemoryModel {
    type Damage = u32;
    type Position = (f32, f32, f32);

    fn damage_received(&self, player: Player) -> Result<Self::Damage, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u32>(MemoryAddress::PlayerHealthOffset, player, 4)
    }

    fn character(&self, player: Player) -> Result<Character, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(MemoryAddress::PlayerCharacterID, player, 4)
            .map(|c| FromPrimitive::from_u16(c).unwrap_or(Character::NotSelected))
    }

    fn xyz(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>> {
        //let x = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetX as usize, 4, handle)?)
        //    .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
        //let y = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetY as usize, 4, handle)?)
        //    .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
        //let z = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetZ as usize, 4, handle)?)
        //    .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");

        //Ok((x, y, z))

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
        player: Player,
    ) -> Result<Option<GroundState>, Box<dyn std::error::Error>> {
        todo!();
        Ok(None)
    }
}

impl RoundMemory for DefaultMemoryModel {
    type FrameCount = u32;
    type RoundCount = u8;
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
