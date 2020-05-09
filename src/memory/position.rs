use byteorder::{LittleEndian, ReadBytesExt};
use read_process_memory::{copy_address, ProcessHandle};

use crate::globals::{MemoryAddresses, Player};

pub fn p1_xyz(handle: &ProcessHandle) -> std::io::Result<(f32, f32, f32)> {
    let x = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetX as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
    let y = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetY as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
    let z = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetZ as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");

    Ok((x, y, z))
}

pub fn p2_xyz(handle: &ProcessHandle) -> std::io::Result<(f32, f32, f32)> {
    let x = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerTwoBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetX as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
    let y = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerTwoBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetY as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
    let z = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerTwoBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetZ as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");

    Ok((x, y, z))
}

pub fn facing(handle: &ProcessHandle, player: Player) -> std::io::Result<u8> {
    let facing_address = match player {
        Player::One => MemoryAddresses::PlayerOneFacing as usize,
        Player::Two => MemoryAddresses::PlayerTwoFacing as usize,
    };

    let facing = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + facing_address, 4, handle)?)
        .read_u8()
        .expect("Failed to parse u8 with little endian byte order");

    Ok(facing)
}