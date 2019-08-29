use byteorder::{LittleEndian, ReadBytesExt};
use read_process_memory::{TryIntoProcessHandle, Pid, ProcessHandle, CopyAddress, copy_address};

use crate::globals::MemoryAddresses;

pub fn p1_xyz(handle: &*mut std::ffi::c_void) -> std::io::Result<(f32, f32, f32)> {
    let x = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetX as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
    let y = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetY as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
    let z = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerOneBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetZ as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");

    Ok((x, y, z))
}

pub fn p2_xyz(handle: &*mut std::ffi::c_void) -> std::io::Result<(f32, f32, f32)> {
    let x = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerTwoBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetX as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
    let y = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerTwoBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetY as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");
    let z = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::PlayerTwoBaseAddress as usize + MemoryAddresses::PlayerCoordinateOffsetZ as usize, 4, handle)?)
        .read_f32::<LittleEndian>().expect("Failed to parse f32 with little endian byte order");

    Ok((x, y, z))
}
