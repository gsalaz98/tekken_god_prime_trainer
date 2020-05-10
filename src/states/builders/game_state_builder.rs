use crate::{memory::MemoryModel, states::game_state::GameState};

pub struct GameStateBuilder<M> {
    memory: M,
}

impl<M: MemoryModel> GameStateBuilder<M> {
    pub fn new(memory: M) -> Self {
        Self { memory }
    }

    pub fn build(self) -> GameState<M> {}
}
