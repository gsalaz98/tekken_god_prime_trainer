use serde::{de::DeserializeOwned, Serialize};

pub trait RoundMemory {
    type FrameCount: Serialize + DeserializeOwned + PartialEq + PartialOrd + Into<u128>;
    type RoundCount: Serialize + DeserializeOwned + PartialEq + PartialOrd + Into<u8>;

    fn round_frame(&self) -> Result<Self::FrameCount, Box<dyn std::error::Error>>;
    fn round(&self) -> Result<Self::RoundCount, Box<dyn std::error::Error>>;
}
