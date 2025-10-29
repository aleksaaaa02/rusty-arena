use bincode::{Encode, Decode};


#[derive(Encode, Decode, Clone, Debug)]
pub enum InputAction {
    RotateLeft,
    RotateRight,
    Shoot,
    Thrust,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct PlayerInput {
    pub id: u32,
    pub action: InputAction,
}