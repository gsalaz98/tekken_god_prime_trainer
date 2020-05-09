use byteorder::{LittleEndian, ReadBytesExt};
use read_process_memory::{copy_address, ProcessHandle};
use serde::{Serialize, de::DeserializeOwned};

use crate::globals::{MemoryAddress, Facing, Player};



pub trait PlayerMemory {
    type Character: Serialize + DeserializeOwned;
    type Damage: Serialize + DeserializeOwned;
    type GroundState: Serialize + DeserializeOwned;
    type Position: Position;

    fn character(&self, player: Player) -> Result<Self::Character, Box<dyn std::error::Error>>;
    fn damage_received(&self, player: Player) -> Result<Self::Damage, Box<dyn std::error::Error>>;
    fn facing(&self, player: Player) -> Result<Facing, Box<dyn std::error::Error>>;
    fn ground_state(&self, player: Player) -> Result<Self::GroundState, Box<dyn std::error::Error>>;
    fn xyz(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>>;
    fn coordinates(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>>;
}

pub trait Position: Serialize + DeserializeOwned {
    type Item;

    fn x() -> Option<Self::Item>;
    fn y() -> Option<Self::Item>;
    fn z() -> Option<Self::Item>;

    fn xyz() -> Option<(Self::Item, Self::Item, Self::Item)>;
}