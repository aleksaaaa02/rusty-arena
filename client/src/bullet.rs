use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BulletNode {
    #[base]
    base: Base<Node2D>,
    x: f32,
    y: f32,
    id: u32,
}

#[godot_api]
impl INode2D for BulletNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base: base,
            id: 0,
            x: 0.0,
            y: 0.0,
        }
    }

    fn ready(&mut self) {
        godot_print!("BulletNode ready, id = {}", self.id);
    }

    fn process(&mut self, delta: f64) {
        let new_position = Vector2 {
            x: self.x,
            y: self.y,
        };
        self.base_mut().set_position(new_position);
    }
}

#[godot_api]
impl BulletNode {
    #[func]
    fn new(id: u32) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            id,
            x: 0.0,
            y: 0.0,
        })
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn update_position(&mut self, position: Vector2) {
        self.x = position.x;
        self.y = position.y;
    }
}
