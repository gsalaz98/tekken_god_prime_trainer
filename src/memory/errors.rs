use crate::globals::Player;
use std::error::Error;

#[derive(Debug)]
pub struct MemoryReadDiagnostics {
    memory_address: usize,
    method_name: String,
}

impl MemoryReadDiagnostics {
    pub fn new(memory_address: usize, method_name: String) -> Self {
        MemoryReadDiagnostics {
            memory_address,
            method_name,
        }
    }
}

#[derive(Debug)]
pub enum MemoryReadErrors {
    FacingTheVoid(Player, MemoryReadDiagnostics),
}

impl std::fmt::Display for MemoryReadErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryReadErrors::FacingTheVoid(player, diag) => {
                write!(f, "{:?} is facing the void. Info: {:#?}", player, diag)
            }
        }
    }
}

impl Error for MemoryReadErrors {}
