use std::io::Cursor;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use read_process_memory::*;

use crate::globals::{MemoryAddress, Player};
use crate::memory::MemoryModel;
use crate::memory::input::InputMemory;
use crate::memory::player::PlayerMemory;
use crate::memory::round::RoundMemory;
use crate::memory::wrappers::CursorWrapper;

#[derive(Clone)]
pub struct DefaultMemoryModel {
    handle: ProcessHandle
}

impl MemoryModel for DefaultMemoryModel {
    fn new(handle: ProcessHandle) -> Self {
        Self {
            handle
        }
    }

    fn read<E, T>(&self, address: usize, length: usize) -> Result<T, Box<dyn std::error::Error>>
    where
        E: ByteOrder, 
        T: CursorWrapper
    {
        let data = copy_address(address, length, &self.handle)
            .expect(&format!("Failed to read memory address: 0x{:X}", address));

        T::read::<E>(Cursor::new(data)).map(|v| v.into())
    }

    fn read_player<E, T>(
        &self, 
        address: MemoryAddress, 
        player: Player, 
        length: usize
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        E: ByteOrder,
        T: CursorWrapper
    {
        let game_address = MemoryAddress::GameAddress as usize + player.base_address() + address as usize;
        self.read::<E, T>(game_address, length)
    }

    fn handle(&self) -> &ProcessHandle {
        &self.handle
    }
}

impl InputMemory for DefaultMemoryModel {
    type GameInput = u16;
    fn inputted_attack(&self, player: Player) -> Result<Self::GameInput, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(MemoryAddress::InputAttack, player, 4)
    }

    fn inputted_direction(&self, player: Player) -> Result<Self::GameInput, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(MemoryAddress::InputDirection, player, 4)
    }
}

impl PlayerMemory for DefaultMemoryModel {
    type Character = u16;
    type Damage = u32;
    type Position = (i64, i64, i64);

    fn damage_received(&self, player: Player) -> Result<Self::Damage, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u32>(MemoryAddress::PlayerHealthOffset, player, 4)
    }

    fn character(&self, player: Player) -> Result<Self::Character, Box<dyn std::error::Error>> {
        self.read_player::<LittleEndian, u16>(MemoryAddress::PlayerCharacterID, player, 4)
    }

    fn xyz(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>> {
        //let x = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetX as usize, 4, handle)?)
        //    .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
        //let y = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetY as usize, 4, handle)?)
        //    .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
        //let z = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetZ as usize, 4, handle)?)
        //    .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");

        //Ok((x, y, z))
    }

    fn coordinates(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>> {

    }

    fn facing(handle: &ProcessHandle, player: Player) -> Result<Facing, Box<dyn std::error::Error>> {
        //let facing_address = match player {
        //    Player::One => MemoryAddresses::PlayerOneFacing as usize,
        //    Player::Two => MemoryAddresses::PlayerTwoFacing as usize,
        //};

        //let facing = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + facing_address, 4, handle)?)
        //    .read_u8()
        //    .expect("Failed to parse u8 with little endian byte order");

        //Ok(facing)
    }

    fn ground_state(&self, player: Player) -> Result<Option<Self::GroundState>, Box<dyn std::error::Error>> {

    }
}

impl RoundMemory for DefaultMemoryModel {
    type FrameCount = u32;
    type RoundCount = u8;
    fn round_frame(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let address = MemoryAddress::GameAddress as usize + MemoryAddress::RoundTimerInFrames as usize;
        self.read::<LittleEndian, Self::FrameCount>(address, 4).map(|v| v.into())
    }

    fn round(&self) -> Result<u8, Box<dyn std::error::Error>> {
        let address = MemoryAddress::GameAddress as usize + MemoryAddress::RoundCount as usize;
        self.read::<LittleEndian, Self::RoundCount>(address, 4)
    }
}