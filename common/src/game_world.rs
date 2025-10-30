use bincode::{Decode, Encode};
use std::{
    collections::{HashMap, binary_heap::Drain},
    time::{Duration, Instant},
};

use crate::packet::PlayerInput;

#[derive(Encode, Decode, Debug, Clone)]
pub struct GameWorld {
    pub players: HashMap<u32, Player>,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub width: f32,
    pub height: f32,
    pub bullet_id_counter: u32,
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
    pub last_shot_ms: u64,
    pub fire_rate_ms: u64,
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
            bullet_id_counter: 0,
        }
    }

    pub fn update(&mut self) {
        let bullet_max_distance = 1000.0;

        self.bullets.iter_mut().for_each(|b| {
            b.x += b.vx;
            b.y += b.vy;
            b.distance_traveled += (b.vx).hypot(b.vy);
        });

        self.bullets
            .retain(|b| b.distance_traveled < bullet_max_distance);

        // here we should update the world per tick
        for player in self.players.values_mut() {
            player.x += player.vx;
            player.y += player.vy;

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
                    let now = GameWorld::current_time_ms();
                    if now >= player.fire_rate_ms + player.last_shot_ms {
                        player.last_shot_ms = now;
                        let speed = 20.0;
                        let id = self.bullet_id_counter;
                        self.bullet_id_counter += 1;
                        let bullet = Bullet {
                            id,
                            owner_id: player.id,
                            x: player.x,
                            y: player.y,
                            vx: speed * player.rotation.cos(),
                            vy: speed * player.rotation.sin(),
                            distance_traveled: 0.0,
                        };

                        self.bullets.push(bullet);
                    }
                }
                crate::packet::InputAction::Thrust => {
                    let force = 2.0;
                    player.vx += force * player.rotation.cos();
                    player.vy += force * player.rotation.sin();
                }
                _ => {}
            }
        } else {
            eprintln!("Entity with id: {player_id} not found!")
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
                vx: 0.0,
                vy: 0.0,
                hp: 100,
                last_shot_ms: 0,
                fire_rate_ms: 100,
            },
        );
    }

    fn current_time_ms() -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");

        now.as_millis() as u64
    }
}
