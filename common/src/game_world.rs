use bincode::{Decode, Encode};
use std::collections::HashMap;

use crate::packet::PlayerInput;


#[derive(Encode, Decode, Debug, Clone)]
pub struct GameWorld {
    pub players: HashMap<u32, Player>,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub width: f32,
    pub height: f32,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub vx: f32,
    pub vy: f32,
    pub hp: u16,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Bullet {
    pub id: u32,
    pub owner_id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32
}


#[derive(Encode, Decode, Debug, Clone)]
pub struct Asteroid {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
}

impl GameWorld {
    pub fn new() -> Self {
       Self { 
        players: HashMap::new(),
        bullets: Vec::new(),
        asteroids: Vec::new(),
        width: 800.0f32,
        height: 800.0f32 } 
    }

    pub fn update(&mut self) {
        // here we should update the world per tick

    }

    pub fn apply_input(&mut self, player_id: u32, input: &PlayerInput) {

    }

    pub fn add_player(&mut self, player_id: u32) {

    }
}