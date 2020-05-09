use enigo::{Enigo, KeyboardControllable};
use serde::{Serialize, Deserialize};

pub enum MemoryAddress {
    GameAddress = 0x140000000,

    PlayerOneBaseAddress = 0x00342B780,
    PlayerTwoBaseAddress = 0x00342E750,
    
    PlayerCharacterID = 0xD8,
    PlayerCoordinateOffsetX = 0x160,
    PlayerCoordinateOffsetY = 0x164,
    PlayerCoordinateOffsetZ = 0x168,
    PlayerThrowingOther = 0x2EC,

    /// Current combo count performed on the player
    OtherComboCount = 0x2F8,
    PlayerHealthOffset = 0x73C,
    InputAttack = 0x16BC,
    InputDirection = 0x16C0,

    RoundCount = 0x00340EEB4,
    RoundTimerInFrames = 0x00340ECE4,
    ThrowTimer = 0x00342EE30,

    PlayerOneFacing = 0x00341D6A0,
    PlayerTwoFacing = 0x00341D6A4
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Player {
    One,
    Two,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Facing {
    Left,
    Right
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Face {
    Down,
    Up,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Feet {
    Away,
    Towards,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Stand {
    Up,
    Crouching,

    BackRoll,

    ForwardRoll,
    ForwardRollStand,
    ForwardRollCrouch,
    ForwardRollLungeAttack,
    ForwardRollLowAttack,
    ForwardRollMidAttack,

    RollLeft,
    RollRight,

    LowAttack,
    MidAttack,

    Special,

    Ukemi,
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub enum InputButton {
    None = 0,
    One = 512,
    Two = 1024,
    Three = 2048,
    Four = 4096,

    OnePlusTwo = 1536,
    OnePlusThree = 2560,
    OnePlusFour = 4608,
    TwoPlusThree = 3072,
    TwoPlusFour = 5120,
    ThreePlusFour = 6144,
    OnePlusTwoPlusThree = 3584,
    OnePlusTwoPlusFour = 5632,
    OnePlusThreePlusFour = 6656,
    TwoPlusThreePlusFour = 7168,
    OnePlusTwoPlusThreePlusFour = 7680,

    Rage = 8192,

    RagePlusOne = 8704,
    RagePlusTwo = 9216,
    RagePlusThree = 10240,
    RagePlusFour = 12288,
    RagePlusOnePlusTwo = 9728,
    RagePlusOnePlusThree = 10752,
    RagePlusOnePlusFour = 12800,
    RagePlusTwoPlusThree = 11264,
    RagePlusTwoPlusFour = 13312,
    RagePlusThreePlusFour = 14336,
    RagePlusOnePlusTwoPlusThree = 11776,
    RagePlusOnePlusTwoPlusFour = 13824,
    RagePlusOnePlusThreePlusFour = 14848,
    RagePlusTwoPlusThreePlusFour = 15360,
    RagePlusOnePlusTwoPlusThreePlusFour = 15872,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Character {
    Paul = 0,
    Law,
    King,
    Yoshimitsu,
    Hwoarang,
    Xiaoyu,
    Jin,
    Bryan,
    Heihachi,
    Kazuya,
    Steve,
    Jack7,
    Asuka,
    DevilJin,
    Feng,
    Lili,
    Dragunov,
    Leo,
    Lars,
    Alisa,
    Claudio,
    Katarina,
    LuckyChloe,
    Shaheen,
    Josie,
    Gigas,
    Kazumi,
    DevilKazumi,
    Nina,
    MasterRaven,
    Lee,
    Bob,
    Akuma,
    Kuma,
    Panda,
    Eddy,
    Eliza,
    Miguel,
    TekkenForce,
    KidKazuya,
    Jack4,
    YoungHeihachi,
    Dummy,
    Geese,
    Noctis,
    Anna,
    Lei,
    Marduk,
    ArmorKing,
    Julia,
    Negan,
    Zafina,
    Ganryu,
    Leroy,
    Fahkumram,

    Unloaded = 71,
    NotSelected = 255
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Character::Paul => write!(f, "Paul"),
            Character::Law => write!(f, "Law"),
            Character::King => write!(f, "King"),
            Character::Yoshimitsu => write!(f, "Yoshimitsu"),
            Character::Hwoarang => write!(f, "Hwoarang"),
            Character::Xiaoyu => write!(f, "Xiaoyu"),
            Character::Jin => write!(f, "Jin"),
            Character::Bryan => write!(f, "Bryan"),
            Character::Heihachi => write!(f, "Heihachi"),
            Character::Kazuya => write!(f, "Kazuya"),
            Character::Steve => write!(f, "Steve"),
            Character::Jack7 => write!(f, "Jack-7"),
            Character::Asuka => write!(f, "Asuka"),
            Character::DevilJin => write!(f, "Devil Jin"),
            Character::Feng => write!(f, "Feng"),
            Character::Lili => write!(f, "Lili"),
            Character::Dragunov => write!(f, "Dragunov"),
            Character::Leo => write!(f, "Leo"),
            Character::Lars => write!(f, "Lars"),
            Character::Alisa => write!(f, "Alisa"),
            Character::Claudio => write!(f, "Claudio"),
            Character::Katarina => write!(f, "Katarina"),
            Character::LuckyChloe => write!(f, "Lucky Chloe"),
            Character::Shaheen => write!(f, "Shaheen"),
            Character::Josie => write!(f, "Josie"),
            Character::Gigas => write!(f, "Gigas"),
            Character::Kazumi => write!(f, "Kazumi"),
            Character::DevilKazumi => write!(f, "Devil Kazumi"),
            Character::Nina => write!(f, "Nina"),
            Character::MasterRaven => write!(f, "Master Raven"),
            Character::Lee => write!(f, "Lee"),
            Character::Bob => write!(f, "Bob"),
            Character::Akuma => write!(f, "Akuma"),
            Character::Kuma => write!(f, "Kuma"),
            Character::Panda => write!(f, "Panda"),
            Character::Eddy => write!(f, "Eddy"),
            Character::Eliza => write!(f, "Eliza"),
            Character::Miguel => write!(f, "Miguel"),
            Character::TekkenForce => write!(f, "Tekken Force"),
            Character::KidKazuya => write!(f, "Kid Kazuya"),
            Character::Jack4 => write!(f, "Jack-4"),
            Character::YoungHeihachi => write!(f, "Young Heihachi"),
            Character::Dummy => write!(f, "Dummy"),
            Character::Geese => write!(f, "Geese"),
            Character::Noctis => write!(f, "Noctis"),
            Character::Anna => write!(f, "Anna"),
            Character::Lei => write!(f, "Lei"),
            Character::Marduk => write!(f, "Marduk"),
            Character::ArmorKing => write!(f, "Armor King"),
            Character::Julia => write!(f, "Julia"),
            Character::Negan => write!(f, "Negan"),
            Character::Zafina => write!(f, "Zafina"),
            Character::Ganryu => write!(f, "Ganryu"),
            Character::Leroy => write!(f, "Leroy"),
            Character::Fahkumram => write!(f, "Fahkumram"),

            Character::Unloaded => write!(f, "Unloaded"),
            Character::NotSelected => write!(f, "Not selected")
        }
    }
}

impl InputButton {
    pub fn to_str(&self) -> &'static str {
        match self {
            InputButton::None => "",
            InputButton::One => "1",
            InputButton::Two => "2",
            InputButton::Three => "3",
            InputButton::Four => "4",
            InputButton::OnePlusTwo => "1+2",
            InputButton::OnePlusThree => "1+3",
            InputButton::OnePlusFour => "1+4",
            InputButton::TwoPlusThree => "2+3",
            InputButton::TwoPlusFour => "2+4",
            InputButton::ThreePlusFour => "3+4",
            InputButton::OnePlusTwoPlusThree => "1+2+3",
            InputButton::OnePlusTwoPlusFour => "1+2+4",
            InputButton::OnePlusThreePlusFour => "1+3+4",
            InputButton::TwoPlusThreePlusFour => "2+3+4",
            InputButton::OnePlusTwoPlusThreePlusFour => "1+2+3+4",
            InputButton::Rage => "RD",

            InputButton::RagePlusOne => "1+RD",
            InputButton::RagePlusTwo => "2+RD",
            InputButton::RagePlusThree => "3+RD",
            InputButton::RagePlusFour => "4+RD",
            InputButton::RagePlusOnePlusTwo => "1+2+RD",
            InputButton::RagePlusOnePlusThree => "1+3+RD",
            InputButton::RagePlusOnePlusFour => "1+4+RD",
            InputButton::RagePlusTwoPlusThree => "2+3+RD",
            InputButton::RagePlusTwoPlusFour => "2+4+RD",
            InputButton::RagePlusThreePlusFour => "3+4+RD",
            InputButton::RagePlusOnePlusTwoPlusThree => "1+2+3+RD",
            InputButton::RagePlusOnePlusTwoPlusFour => "1+2+4+RD",
            InputButton::RagePlusOnePlusThreePlusFour => "1+3+4+RD",
            InputButton::RagePlusTwoPlusThreePlusFour => "2+3+4+RD",
            InputButton::RagePlusOnePlusTwoPlusThreePlusFour => "1+2+3+4+RD",
        }
    }

    pub fn to_input_key(&self, player: &Player) -> &'static str {
        match player {
            Player::One => match self {
                InputButton::None => "",
                InputButton::One => "u",
                InputButton::Two => "i",
                InputButton::Three => "j",
                InputButton::Four => "k",
                InputButton::OnePlusTwo => "ui",
                InputButton::OnePlusThree => "uj",
                InputButton::OnePlusFour => "uk",
                InputButton::TwoPlusThree => "ij",
                InputButton::TwoPlusFour => "ik",
                InputButton::ThreePlusFour => "jk",
                InputButton::OnePlusTwoPlusThree => "uij",
                InputButton::OnePlusTwoPlusFour => "uik",
                InputButton::OnePlusThreePlusFour => "ujk",
                InputButton::TwoPlusThreePlusFour => "ijk",
                InputButton::OnePlusTwoPlusThreePlusFour => "uijk",
                InputButton::Rage => "o",

                InputButton::RagePlusOne => "ou",
                InputButton::RagePlusTwo => "oi",
                InputButton::RagePlusThree => "oj",
                InputButton::RagePlusFour => "ok",
                InputButton::RagePlusOnePlusTwo => "oui",
                InputButton::RagePlusOnePlusThree => "ouj",
                InputButton::RagePlusOnePlusFour => "ouk",
                InputButton::RagePlusTwoPlusThree => "oij",
                InputButton::RagePlusTwoPlusFour => "oik",
                InputButton::RagePlusThreePlusFour => "ojk",
                InputButton::RagePlusOnePlusTwoPlusThree => "ouij",
                InputButton::RagePlusOnePlusTwoPlusFour => "ouik",
                InputButton::RagePlusOnePlusThreePlusFour => "oujk",
                InputButton::RagePlusTwoPlusThreePlusFour => "oijk",
                InputButton::RagePlusOnePlusTwoPlusThreePlusFour => "ouijk",
            },

            Player::Two => match self {
                InputButton::None => "",
                InputButton::One => "7",
                InputButton::Two => "8",
                InputButton::Three => "5",
                InputButton::Four => "6",
                InputButton::OnePlusTwo => "78",
                InputButton::OnePlusThree => "75",
                InputButton::OnePlusFour => "76",
                InputButton::TwoPlusThree => "85",
                InputButton::TwoPlusFour => "86",
                InputButton::ThreePlusFour => "56",
                InputButton::OnePlusTwoPlusThree => "785",
                InputButton::OnePlusTwoPlusFour => "786",
                InputButton::OnePlusThreePlusFour => "756",
                InputButton::TwoPlusThreePlusFour => "856",
                InputButton::OnePlusTwoPlusThreePlusFour => "7856",
                InputButton::Rage => "-",

                InputButton::RagePlusOne => "-7",
                InputButton::RagePlusTwo => "-8",
                InputButton::RagePlusThree => "-5",
                InputButton::RagePlusFour => "-6",
                InputButton::RagePlusOnePlusTwo => "-78",
                InputButton::RagePlusOnePlusThree => "-75",
                InputButton::RagePlusOnePlusFour => "-76",
                InputButton::RagePlusTwoPlusThree => "-85",
                InputButton::RagePlusTwoPlusFour => "-86",
                InputButton::RagePlusThreePlusFour => "-56",
                InputButton::RagePlusOnePlusTwoPlusThree => "-785",
                InputButton::RagePlusOnePlusTwoPlusFour => "-786",
                InputButton::RagePlusOnePlusThreePlusFour => "-756",
                InputButton::RagePlusTwoPlusThreePlusFour => "-856",
                InputButton::RagePlusOnePlusTwoPlusThreePlusFour => "-7856",
            }
        }
    }

    pub fn input_attack(&self, player: Player, previous_button: Option<InputButton>) {
        let input_keys = self.to_input_key(&player);
        let mut enigo = Enigo::new();

        if previous_button.is_some() {
            let prev = previous_button.unwrap();

            if input_keys != prev.to_input_key(&player) {
                for key in prev.to_input_key(&player).chars() {
                    enigo.key_up(enigo::Key::Layout(key));
                }

                for key in self.to_input_key(&player).chars() {
                    enigo.key_down(enigo::Key::Layout(key));
                }

                return;
            }
        }
        else {
            for key in self.to_input_key(&player).chars() {
                enigo.key_down(enigo::Key::Layout(key));
            }
        }
    }
}

impl InputDirection {
    pub fn to_str(&self) -> &'static str {
        match self {
            InputDirection::Neutral => " ",
            InputDirection::Back => "b",
            InputDirection::DownBack => "d/b",
            InputDirection::Down => "d",
            InputDirection::DownForward => "d/f",
            InputDirection::Forward => "f",
            InputDirection::UpForward => "u/f",
            InputDirection::Up => "u",
            InputDirection::UpBack => "u/b",
        }
    }

    /// Takes player and player side to calculate which button(s) to press
    pub fn to_input_key(&self, player: &Player, side: Player) -> &'static str {
        match player {
            Player::One => match side {
                Player::One => match self {
                    InputDirection::Neutral => "",
                    InputDirection::Back => "a",
                    InputDirection::DownBack => "as",
                    InputDirection::Down => "s",
                    InputDirection::DownForward => "sd",
                    InputDirection::Forward => "d",
                    InputDirection::UpForward => "wd",
                    InputDirection::Up => "w",
                    InputDirection::UpBack => "wa",
                },
                Player::Two => match self {
                    InputDirection::Neutral => "",
                    InputDirection::Back => "d",
                    InputDirection::DownBack => "ds",
                    InputDirection::Down => "s",
                    InputDirection::DownForward => "sa",
                    InputDirection::Forward => "a",
                    InputDirection::UpForward => "wa",
                    InputDirection::Up => "w",
                    InputDirection::UpBack => "wd",
                }
            }
            Player::Two => match side {
                Player::One => match self {
                    InputDirection::Neutral => "",
                    InputDirection::Back => "3",
                    InputDirection::DownBack => "23",
                    InputDirection::Down => "2",
                    InputDirection::DownForward => "24",
                    InputDirection::Forward => "4",
                    InputDirection::UpForward => "14",
                    InputDirection::Up => "1",
                    InputDirection::UpBack => "13",
                },
                Player::Two => match self {
                    InputDirection::Neutral => "",
                    InputDirection::Back => "4",
                    InputDirection::DownBack => "24",
                    InputDirection::Down => "2",
                    InputDirection::DownForward => "23",
                    InputDirection::Forward => "3",
                    InputDirection::UpForward => "13",
                    InputDirection::Up => "1",
                    InputDirection::UpBack => "14",
                }
            }
        }
    }

    pub fn input_direction(&self, player: Player, side: Player, previous_side: Option<Player>, previous_direction: Option<InputDirection>) {
        let input_keys = self.to_input_key(&player, side);
        let mut enigo = Enigo::new();

        if previous_direction.is_some() && previous_side.is_some() {
            let prev = previous_direction.unwrap().to_input_key(&player, previous_side.unwrap());

            if prev != input_keys {
                for key in prev.chars() {
                    enigo.key_up(enigo::Key::Layout(key));
                }

                for key in input_keys.chars() {
                    enigo.key_down(enigo::Key::Layout(key));
                }
            }
        }
        else {
            for key in input_keys.chars() {
                enigo.key_down(enigo::Key::Layout(key));
            }
        }
    }
}

impl From<usize> for InputDirection {
    fn from(value: usize) -> InputDirection {
        match value {
            32 => InputDirection::Neutral,
            0 => InputDirection::Neutral,

            16 => InputDirection::Back,
            2 => InputDirection::DownBack,
            4 => InputDirection::Down,
            8 => InputDirection::DownForward,
            64 => InputDirection::Forward,
            512 => InputDirection::UpForward,
            256 => InputDirection::Up,
            128 => InputDirection::UpBack,

            _ => panic!(format!("Received unknown value for input direction with value {}", value))
        }
    }
}

impl From<usize> for InputButton {
    fn from(value: usize) -> InputButton {
        match value {
            0 => InputButton::None,
            512 => InputButton::One,
            1024 => InputButton::Two,
            2048 => InputButton::Three,
            4096 => InputButton::Four,

            1536 => InputButton::OnePlusTwo,
            2560 => InputButton::OnePlusThree,
            4608 => InputButton::OnePlusFour,
            3072 => InputButton::TwoPlusThree,
            5120 => InputButton::TwoPlusFour,
            6144 => InputButton::ThreePlusFour,
            3584 => InputButton::OnePlusTwoPlusThree,
            5632 => InputButton::OnePlusTwoPlusFour,
            6656 => InputButton::OnePlusThreePlusFour,
            7168 => InputButton::TwoPlusThreePlusFour,
            7680 => InputButton::OnePlusTwoPlusThreePlusFour,

            8192 => InputButton::Rage,
            8704 => InputButton::RagePlusOne,
            9216 => InputButton::RagePlusTwo,
            10240 => InputButton::RagePlusThree,
            12288 => InputButton::RagePlusFour,
            9728 => InputButton::RagePlusOnePlusTwo,
            10752 => InputButton::RagePlusOnePlusThree,
            12800 => InputButton::RagePlusOnePlusFour,
            11264 => InputButton::RagePlusTwoPlusThree,
            13312 => InputButton::RagePlusTwoPlusFour,
            14336 => InputButton::RagePlusThreePlusFour,
            11776 => InputButton::RagePlusOnePlusTwoPlusThree,
            13824 => InputButton::RagePlusOnePlusTwoPlusFour,
            14848 => InputButton::RagePlusOnePlusThreePlusFour,
            15360 => InputButton::RagePlusTwoPlusThreePlusFour,
            15872 => InputButton::RagePlusOnePlusTwoPlusThreePlusFour,

            _ => panic!(format!("Unknown input button encountered: {}", value))
        }
    }
}

impl Player {
    pub fn base_address(&self) -> usize {
        match self {
            Player::One => MemoryAddress::PlayerOneBaseAddress as usize,
            Player::Two => MemoryAddress::PlayerTwoBaseAddress as usize
        }
    }
}

impl std::ops::Not for Player {
    type Output = Player;

    fn not(self) -> Self::Output {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One
        }
    }
}