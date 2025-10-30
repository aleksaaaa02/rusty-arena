use bincode::{Decode, Encode};
use rand::Rng;
use std::collections::{HashMap, HashSet};

use crate::packet::PlayerInput;

#[derive(Encode, Decode, Debug, Clone)]
pub struct Bounds {
    pub west: f32,
    pub east: f32,
    pub north: f32,
    pub south: f32,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct GameWorld {
    pub players: HashMap<u32, Player>,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub width: f32,
    pub height: f32,
    pub bullet_id_counter: u32,
    pub asteroid_id_counter: u32,
    pub last_spawn_asteroid: u64,
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
    pub distance_travaled: f32,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            bullets: Vec::new(),
            asteroids: Vec::new(),
            width: 1600.0,
            height: 1200.0,
            bullet_id_counter: 0,
            asteroid_id_counter: 0,
            last_spawn_asteroid: 0,
        }
    }

    pub fn update(&mut self) {
        // Update Bullets in world
        let bullet_max_distance = 1000.0;
        let asteroids_max_distance: f32 = 1000.0;
        let asteroid_speed = 1.5;

        self.bullets.iter_mut().for_each(|b| {
            b.x += b.vx;
            b.y += b.vy;
            b.distance_traveled += (b.vx).hypot(b.vy);
        });

        self.bullets
            .retain(|b| b.distance_traveled < bullet_max_distance);

        self.asteroids.iter_mut().for_each(|a| {
            a.x += a.vx * asteroid_speed;
            a.y += a.vy * asteroid_speed;

            a.distance_travaled += (a.vx).hypot(a.vy);
        });

        self.asteroids
            .retain(|a| a.distance_travaled < asteroids_max_distance);

        // Update players
        for player in self.players.values_mut() {
            player.x += player.vx;
            player.y += player.vy;

            player.x = player.x.clamp(-800.0, 800.0);
            player.y = player.y.clamp(-600.0, 600.0);

            // Friction / damping to slow down
            player.vx *= 0.9;
            player.vy *= 0.9;
        }

        // Check for collision
        let mut bullets_to_remove = HashSet::new();
        let mut asteroids_to_remove = HashSet::new();
        let mut players_hit = HashSet::new();

        for bullet in &self.bullets {
            for asteroid in self.asteroids.iter_mut() {
                let dx = bullet.x - asteroid.x;
                let dy = bullet.y - asteroid.y;
                let dist_sq = dx * dx + dy * dy;
                let collision_radius = 30.0;
                if dist_sq < collision_radius * collision_radius {
                    asteroids_to_remove.insert(asteroid.id);
                    bullets_to_remove.insert(bullet.id);
                }
            }

            for player in self.players.values_mut() {
                if bullet.owner_id == player.id {
                    continue;
                }

                let dx = bullet.x - player.x;
                let dy = bullet.y - player.y;
                let dist_sq = dx * dx + dy * dy;
                let collision_radius = 20.0;
                if dist_sq < collision_radius * collision_radius {
                    player.hp = player.hp.saturating_sub(20);
                    bullets_to_remove.insert(bullet.id);
                    players_hit.insert(player.id);
                }
            }
        }

        for asteroid in &self.asteroids {
            for player in self.players.values_mut() {
                let dx = asteroid.x - player.x;
                let dy = asteroid.y - player.y;
                let dist_sq = dx * dx + dy * dy;
                let collision_radius = 30.0;
                if dist_sq < collision_radius * collision_radius {
                    player.hp = player.hp.saturating_sub(20);
                    asteroids_to_remove.insert(asteroid.id);
                    players_hit.insert(player.id);
                }
            }
        }

        self.bullets.retain(|b| !bullets_to_remove.contains(&b.id));
        self.asteroids.retain(|b| !asteroids_to_remove.contains(&b.id));

        // Kill / Respawn
        for id in players_hit {
            if let Some(player) = self.players.get_mut(&id) {
                if player.hp == 0 {
                    player.x = 0.0;
                    player.y = 0.0;
                    player.vx = 0.0;
                    player.vy = 0.0;
                    player.hp = 100;
                }
            }
        }

        let now = GameWorld::current_time_ms();
        if now - self.last_spawn_asteroid > 1000 {
            println!("woosh");
            let (pos, dir) = GameWorld::spawn_asteroid();
            let id = self.asteroid_id_counter;
            self.asteroid_id_counter += 1;
            self.asteroids.push(Asteroid {
                id: id,
                x: pos.0,
                y: pos.1,
                vx: dir.0,
                vy: dir.1,
                radius: 1.0,
                distance_travaled: 0.0,
            });
            self.last_spawn_asteroid = now;
        }
    }

    pub fn apply_input(&mut self, player_id: u32, input: &PlayerInput) {
        if let Some(player) = self.players.get_mut(&player_id) {
            match input.action {
                crate::packet::InputAction::RotateLeft => {
                    player.rotation -= 0.05;
                }
                crate::packet::InputAction::RotateRight => {
                    player.rotation += 0.05;
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
                    let force = 1.0;
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

    fn spawn_asteroid() -> ((f32, f32), (f32, f32)) {
        let mut rng = rand::rng();
        let margin = 50.0;
        let side = rng.random_range(0..=3);
        let spawn_pos = match side {
            0 => (rng.random_range(-800.0..800.0), 600.0 + margin), // top
            1 => (rng.random_range(-800.0..800.0), -600.0 - margin), // bottom
            2 => (-800.0 - margin, rng.random_range(-600.0..600.0)), // left
            3 => (800.0 + margin, rng.random_range(-600.0..600.0)), // right
            _ => (0.0, 0.0),
        };

        let players = vec![(0.0, 0.0), (100.0, 50.0), (-200.0, -100.0)];
        let target_player = players[rng.random_range(0..players.len())];

        let dx = target_player.0 - spawn_pos.0;
        let dy = target_player.1 - spawn_pos.1;
        let distance: f32 = ((dx * dx + dy * dy) as f32).sqrt();
        let direction = (dx / distance, dy / distance);

        (spawn_pos, direction)
    }

    fn current_time_ms() -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");

        now.as_millis() as u64
    }
}
