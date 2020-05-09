use serde::{Serialize, de::DeserializeOwned};
use crate::globals::Player;

pub trait InputMemory {
    type GameInput: Serialize + DeserializeOwned;

    fn inputted_attack(&self, player: Player) -> Result<Self::GameInput, Box<dyn std::error::Error>>;
    fn inputted_direction(&self, player: Player) -> Result<Self::GameInput, Box<dyn std::error::Error>>;
}