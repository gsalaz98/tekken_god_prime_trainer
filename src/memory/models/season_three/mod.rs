use super::MemoryModel;
use read_process_memory::ProcessHandle;

#[derive(Clone)]
pub struct V3Dot33 {
    pub(crate) handle: ProcessHandle,
}

impl MemoryModel for V3Dot33 {
    fn new(handle: ProcessHandle) -> Self {
        Self { handle }
    }

    fn player_base_address(&self, player: crate::globals::Player)                  -> usize {
        todo!()
    }

    fn input_attack_address(&self, player: crate::globals::Player)          -> usize {
        todo!()
    }

    fn input_direction_address(&self, player: crate::globals::Player)       -> usize {
        todo!()
    }

    fn player_health_address(&self, player: crate::globals::Player)         -> usize {
        todo!()
    }

    fn player_character_id_address(&self, player: crate::globals::Player)   -> usize {
        todo!()
    }

    fn player_coordinate_x(&self, player: crate::globals::Player)           -> usize {
        todo!()
    }

    fn player_coordinate_y(&self, player: crate::globals::Player)           -> usize {
        todo!()
    }

    fn player_coordinate_z(&self, player: crate::globals::Player)           -> usize {
        todo!()
    }

    fn player_facing_address(&self, player: crate::globals::Player)         -> usize {
        todo!()
    }

    fn round_count_address(&self)                           -> usize {
        todo!()
    }

    fn round_timer_address(&self)                           -> usize {
        todo!()
    }

    fn handle(&self) -> &ProcessHandle {
        todo!()
    }
}