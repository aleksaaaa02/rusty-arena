use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BulletNode {
    base: Base<Node2D>,
    id: u32,
}

#[godot_api]
impl INode2D for BulletNode {
    fn init(base: Base<Node2D>) -> Self {
        Self { base, id: 0 }
    }

    fn ready(&mut self) {
        godot_print!("BulletNode ready, id = {}", self.id);
    }

    fn process(&mut self, delta: f64) {}
}

#[godot_api]
impl BulletNode {
    #[func]
    fn new(id: u32) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { base, id })
    }
}
