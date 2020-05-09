
pub struct PlayerState {
    player: Player,

    x: f32,
    y: f32,
    z: f32,

    input_attack: u16,
    input_direction: u16,
    damage_received: u32,
    facing: u8,

    last_update: f64,

    #[serde(skip)]
    char_id: M::Character,
    #[serde(skip)]
    char_id: M::Character,

    #[serde(skip)]
    p1_previous_button: globals::InputButton,
    #[serde(skip)]
    p1_previous_direction: globals::InputDirection,
    #[serde(skip)]
    p1_previous_facing: globals::Player,

    #[serde(skip)]
    p2_previous_button: globals::InputButton,
    #[serde(skip)]
    p2_previous_direction: globals::InputDirection,
    #[serde(skip)]
    p2_previous_facing: globals::Player,
}