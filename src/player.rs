use byteorder::{LittleEndian, ReadBytesExt};
use read_process_memory::copy_address;

use crate::globals::MemoryAddresses;

pub fn get_damage_received(handle: &*mut std::ffi::c_void, player_base_address: MemoryAddresses) -> std::io::Result<u32> {
    let damage = std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + player_base_address as usize + MemoryAddresses::PlayerHealthOffset as usize, 4, handle)?)
        .read_u32::<LittleEndian>()
        .expect("Failed to parse f32 with little endian byte order");

    Ok(damage)
}

pub fn get_player_char_id(handle: &*mut std::ffi::c_void, player_base_address: MemoryAddresses) -> std::io::Result<u16> {
    Ok(std::io::Cursor::new(copy_address(MemoryAddresses::GameAddress as usize + player_base_address as usize + MemoryAddresses::PlayerCharacterID as usize, 4, handle)?)
        .read_u16::<LittleEndian>()
        .expect("Failed to parse f32 with little endian byte order"))
}