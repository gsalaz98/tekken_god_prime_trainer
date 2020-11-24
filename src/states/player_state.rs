use crate::{memory::MemoryModel, globals::{Character, Player, Facing}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub(crate) player: Player,
    pub(crate) character: Character,

    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,

    pub(crate) input_attack: u16,
    pub(crate) input_direction: u16,
    pub(crate) damage_received: u32,
    pub(crate) facing: u8,

    pub(crate) last_update: u128,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub(crate) screen_name: Option<String>
}

impl PlayerState {
    pub fn new<M: MemoryModel>(memory: &M, player: Player) -> Self {
        let xyz: (f32, f32, f32) = memory.xyz(player).unwrap().into();

        Self { 
            player,
            character: memory.character(player).unwrap(),

            x: xyz.0,
            y: xyz.1,
            z: xyz.2,

            input_attack: memory.inputted_attack(player).unwrap().into(),
            input_direction: memory.inputted_direction(player).unwrap().into(),
            damage_received: memory.damage_received(player).unwrap().into(),
            facing: match memory.facing(player).unwrap() {
                Facing::Left => 0,
                Facing::Right => 1
            },

            last_update: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        }
    }

    pub fn update<M: MemoryModel>(&mut self, memory: &M) {
        let xyz: (f32, f32, f32) = memory.xyz(self.player).unwrap().into();

        self.x = xyz.0;
        self.y = xyz.1;
        self.z = xyz.2;

        self.input_attack = memory.inputted_attack(self.player).unwrap().into();
        self.input_direction = memory.inputted_direction(self.player).unwrap().into();
        self.damage_received = memory.damage_received(self.player).unwrap().into();
        self.facing = match memory.facing(self.player).unwrap() {
            Facing::Left => 0,
            Facing::Right => 1
        };

        self.last_update = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    pub fn character(&self) -> Character {
        self.character
    }
}

impl PlayerInfo {
    fn screen_name(&self) -> &Option<String> {
        &self.screen_name
    }
}
