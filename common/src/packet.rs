use bincode::{Encode, Decode};
use serde::{Deserialize, Serialize};


#[derive(Encode, Decode, Clone, Debug, Copy)]
pub enum InputAction {
    RotateLeft,
    RotateRight,
    Shoot,
    Thrust,
    Hello,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct PlayerInput {
    pub id: u32,
    pub seq: u32,
    pub action: InputAction,
}

impl InputAction {
    pub fn get_input_code_from_action(&self) -> u32 {
        match &self {
            Self::RotateLeft => 1,
            Self::RotateRight => 2,
            Self::Thrust => 3,
            Self::Shoot => 4,
            Self::Hello => 5,
        }
    }

    pub fn get_action_from_code(code: u32) -> InputAction {
        match code {
            1 => Self::RotateLeft,
            2 => Self::RotateRight,
            3 => Self::Thrust,
            4 => Self::Shoot,
            5 => Self::Hello,
            _ => Self::Hello
        }
    }

}

