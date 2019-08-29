
pub enum MemoryAddresses {
    GameAddress = 0x140000000,

    PlayerOneBaseAddress = 0x00342B780,
    PlayerTwoBaseAddress = 0x00342E750,
    
    PlayerCoordinateOffsetX = 0x160,
    PlayerCoordinateOffsetY = 0x164,
    PlayerCoordinateOffsetZ = 0x168,

    RoundCount = 0x00340EEB4,
    RoundTimerInFrames = 0x00340ECE4
}