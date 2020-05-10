use crate::globals::{Character, Facing, GroundState, Player};
use serde::{de::DeserializeOwned, Serialize};

pub trait PlayerMemory {
    type Damage: Serialize + DeserializeOwned;
    type Position: Serialize + DeserializeOwned;

    fn character(&self, player: Player) -> Result<Character, Box<dyn std::error::Error>>;
    fn damage_received(&self, player: Player) -> Result<Self::Damage, Box<dyn std::error::Error>>;
    fn facing(&self, player: Player) -> Result<Facing, Box<dyn std::error::Error>>;
    fn ground_state(
        &self,
        player: Player,
    ) -> Result<Option<GroundState>, Box<dyn std::error::Error>>;
    fn xyz(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>>;
    fn coordinates(&self, player: Player) -> Result<Self::Position, Box<dyn std::error::Error>>;
}
