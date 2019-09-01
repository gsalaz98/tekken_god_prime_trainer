use byteorder::{LittleEndian, ReadBytesExt};
use read_process_memory::{TryIntoProcessHandle, Pid, ProcessHandle, CopyAddress, copy_address};

use crate::globals::MemoryAddresses;

pub fn get_inputted_attack(handle: &*mut std::ffi::c_void, player_base_address: MemoryAddresses) -> std::io::Result<u16> {
    let attack_input = std::io::Cursor::new(
        copy_address(
            MemoryAddresses::GameAddress as usize + player_base_address as usize + MemoryAddresses::InputAttack as usize, 4, handle)?)
        .read_u16::<LittleEndian>()
        .expect("Failed to parse u16 with little endian byte order");

    Ok(attack_input)
}

pub fn get_inputted_direction(handle: &*mut std::ffi::c_void, player_base_address: MemoryAddresses) -> std::io::Result<u16> {
    let input_direction = std::io::Cursor::new(
        copy_address(
            MemoryAddresses::GameAddress as usize + player_base_address as usize + MemoryAddresses::InputDirection as usize, 4, handle)?)
        .read_u16::<LittleEndian>()
        .expect("Failed to parse u16 with little endian byte order");

    Ok(input_direction)
}