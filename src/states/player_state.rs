use crate::globals::{Character, Player};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait PlayerState: Serialize + DeserializeOwned {}
pub trait PlayerInfo<'a>: Serialize + Deserialize<'a> {}

#[derive(Clone, Serialize, Deserialize)]
pub struct DefaultPlayerState {
    player: Player,

    x: f32,
    y: f32,
    z: f32,

    input_attack: u16,
    input_direction: u16,
    damage_received: u32,
    facing: u8,

    last_update: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DefaultPlayerInfo {
    #[serde(skip)]
    character: Character,
}

impl PlayerState for DefaultPlayerState {}
impl<'a> PlayerInfo<'a> for DefaultPlayerInfo {}
