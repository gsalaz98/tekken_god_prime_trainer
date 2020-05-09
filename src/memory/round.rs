use serde::{Serialize, de::DeserializeOwned};

pub trait RoundMemory {
    type FrameCount: Serialize + DeserializeOwned + PartialEq + PartialOrd;
    type RoundCount: Serialize + DeserializeOwned + PartialEq + PartialOrd;

    fn round_frame(&self) -> Result<Self::FrameCount, Box<dyn std::error::Error>>;
    fn round(&self) -> Result<Self::RoundCount, Box<dyn std::error::Error>>;
}