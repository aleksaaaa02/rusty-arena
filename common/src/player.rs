use bincode::{Decode, Encode};

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
    pub last_processed_input_seq: u32,
}

impl Player {
    pub fn new(id: u32) -> Self {
        Player {
            id: id,
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            vx: 0.0,
            vy: 0.0,
            hp: 100,
            last_shot_ms: 0,
            fire_rate_ms: 200,
            last_processed_input_seq: 0,
        }
    }

    pub fn update_player_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        self.x = self.x.clamp(-800.0, 800.0);
        self.y = self.y.clamp(-600.0, 600.0);

        self.vx *= 0.9;
        self.vy *= 0.9;
    }
}