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
    pub vy: f32,
    pub distance_traveled: f32,
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
            height: 800.0f32,
        }
    }

    pub fn update(&mut self) {
        let max_distance = 1000.0;

        self.bullets.iter_mut().for_each(|b| {
            b.x += b.vx * 0.016;
            b.y += b.vy * 0.016;
            b.distance_traveled += (b.vx * 0.16).hypot(b.vy * 0.016);
        });

        self.bullets.retain(|b| b.distance_traveled < max_distance);

        // here we should update the world per tick
        for player in self.players.values_mut() {
            player.x += player.vx * 0.016;
            player.y += player.vy * 0.016;

            // Optional: wrap around the world boundaries
            if player.x < 0.0 {
                player.x += self.width;
            }
            if player.x > self.width {
                player.x -= self.width;
            }
            if player.y < 0.0 {
                player.y += self.height;
            }
            if player.y > self.height {
                player.y -= self.height;
            }

            // Friction / damping to slow down
            player.vx *= 0.9;
            player.vy *= 0.9;
        }
    }

    pub fn apply_input(&mut self, player_id: u32, input: &PlayerInput) {
        if let Some(player) = self.players.get_mut(&player_id) {
            match input.action {
                crate::packet::InputAction::RotateLeft => {
                    player.rotation -= 0.1;
                }
                crate::packet::InputAction::RotateRight => {
                    player.rotation += 0.1;
                }
                crate::packet::InputAction::Shoot => {
                    let speed = 500.0;
                    let bullet = Bullet {
                        id: 0,
                        owner_id: player.id,
                        x: player.x,
                        y: player.y,
                        vx: speed * player.rotation.cos(),
                        vy: speed * player.rotation.sin(),
                        distance_traveled: 0.0,
                    };

                    self.bullets.push(bullet);
                }
                crate::packet::InputAction::Thrust => {
                    let force = 2.0;
                    player.vx += force * player.rotation.cos();
                    player.vy += force * player.rotation.sin();
                }
                _ => {}
            }
        }
    }

    pub fn add_player(&mut self, player_id: u32) {
        self.players.insert(
            player_id,
            Player {
                id: player_id,
                x: 0.0,
                y: 0.0,
                rotation: 0.0,
                vx: 800.0 / 2.0,
                vy: 800.0 / 2.0,
                hp: 100,
            },
        );
    }
}
