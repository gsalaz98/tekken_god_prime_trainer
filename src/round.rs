use byteorder::{LittleEndian, ReadBytesExt};
use read_process_memory::*;
use crate::globals::MemoryAddresses;

pub fn get_round_frame_count(handle: &*mut std::ffi::c_void) -> std::io::Result<u32> {
    Ok(std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::RoundTimerInFrames as usize, 4, handle)?)
        .read_u32::<LittleEndian>()
        .expect("Failed to parse round frame count with little endian"))
}

pub fn get_round(handle: &*mut std::ffi::c_void) -> std::io::Result<u8> {
    Ok(std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + MemoryAddresses::RoundCount as usize, 4, handle)?)
        .read_u8()
        .expect("Failed to parse round frame count with little endian"))
}
