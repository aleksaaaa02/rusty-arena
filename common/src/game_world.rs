use bincode::{Decode, Encode};
use std::collections::{HashMap, HashSet};

use crate::{
    asteroid::Asteroid, bullet::Bullet, packet::PlayerInput, player::Player, utils::current_time_ms,
};

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
        let now = current_time_ms();

        // Update Bullets in world
        let bullet_max_distance = 1000.0;
        let asteroids_max_distance: f32 = 1000.0;

        // update entities

        self.bullets.iter_mut().for_each(|b| b.update_position());

        self.bullets
            .retain(|b| b.distance_traveled < bullet_max_distance);

        self.asteroids.iter_mut().for_each(|a| a.update_position());

        self.asteroids
            .retain(|a| a.distance_traveled < asteroids_max_distance);

        self.players
            .iter_mut()
            .for_each(|(_, player)| player.update_player_position());

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
        self.asteroids
            .retain(|b| !asteroids_to_remove.contains(&b.id));

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

        // spawn asteroids
        if now - self.last_spawn_asteroid > 1000 {
            let id = self.asteroid_id_counter;
            self.asteroid_id_counter += 1;
            self.asteroids.push(Asteroid::new(id));
            self.last_spawn_asteroid = now;
        }
    }

    pub fn apply_input(&mut self, player_id: u32, input: &PlayerInput) {
        if let Some(player) = self.players.get_mut(&player_id) {
            if input.seq  <= player.last_processed_input_seq {
                return;
            }

            match input.action {
                crate::packet::InputAction::RotateLeft => {
                    player.rotation -= 0.05;
                }
                crate::packet::InputAction::RotateRight => {
                    player.rotation += 0.05;
                }
                crate::packet::InputAction::Shoot => {
                    let now = current_time_ms();
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
                crate::packet::InputAction::Hello => {
                    // initial handshake so screen don't freeze on game init
                    // just letting server know that it should broadcast it's state to a new player
                }
            }
            player.last_processed_input_seq = input.seq;
        } else {
            eprintln!("Entity with id: {player_id} not found!")
        }
    }

    pub fn add_player(&mut self, player_id: u32) {
        self.players.insert(player_id, Player::new(player_id));
    }
}
