mod input;
mod player;
mod round;
mod wrappers;

mod models;

use byteorder::ByteOrder;
use read_process_memory::ProcessHandle;

use crate::globals::{MemoryAddress, Player};
pub use input::InputMemory;
pub use player::PlayerMemory;
pub use round::RoundMemory;
pub use wrappers::CursorWrapper;

pub trait MemoryModel: Clone + InputMemory + PlayerMemory + RoundMemory {
    fn new(handle: ProcessHandle) -> Self;

    fn read<E, T>(&self, address: usize, length: usize) -> Result<T, Box<dyn std::error::Error>>
    where
        E: ByteOrder, 
        T: CursorWrapper;

    fn read_player<E, T>(
        &self, 
        address: MemoryAddress, 
        player: Player, 
        length: usize
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        E: ByteOrder,
        T: CursorWrapper;

    fn handle(&self) -> &ProcessHandle;
}