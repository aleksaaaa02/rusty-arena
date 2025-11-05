use bincode::{Decode, Encode};
use rand::Rng;

#[derive(Encode, Decode, Debug, Clone)]
pub struct Asteroid {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub asteroid_speed: f32,
    pub distance_traveled: f32,
}

impl Asteroid {
    pub fn update_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.distance_traveled += (self.vx).hypot(self.vy);
    }

    pub fn new(id: u32) -> Self {
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

        Self {
            id: id,
            x: spawn_pos.0,
            y: spawn_pos.1,
            vx: direction.0,
            vy: direction.1,
            radius: 1.0,
            asteroid_speed: 1.5,
            distance_traveled: 0.0,
        }
    }
}
