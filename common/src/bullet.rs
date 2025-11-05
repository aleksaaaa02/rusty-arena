use bincode::{Decode, Encode};

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

impl Bullet {
    pub fn new(id: u32) {}

    pub fn update_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.distance_traveled += (self.vx).hypot(self.vy);
    }
}
