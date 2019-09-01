
pub enum MemoryAddresses {
    GameAddress = 0x140000000,

    PlayerOneBaseAddress = 0x00342B780,
    PlayerTwoBaseAddress = 0x00342E750,
    
    PlayerCoordinateOffsetX = 0x160,
    PlayerCoordinateOffsetY = 0x164,
    PlayerCoordinateOffsetZ = 0x168,
    PlayerThrowingOther = 0x2EC,
    PlayerHealthOffset = 0x73C,
    InputAttack = 0x16BC,
    InputDirection = 0x16C0,

    RoundCount = 0x00340EEB4,
    RoundTimerInFrames = 0x00340ECE4,
    ThrowTimer = 0x00342EE30,
}


pub enum InputDirection {
    Neutral = 1 << 5,

    Back = 1 << 4,
    DownBack = 1 << 1,
    Down = 1 << 2,
    DownForward = 1 << 3,
    Forward = 1 << 6,
    UpForward = 1 << 9,
    Up = 1 << 8,
    UpBack = 1 << 7,
}