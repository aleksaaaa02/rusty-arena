use godot::{
    classes::{Camera2D, ICamera2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Camera2D)]
pub struct CameraNode {
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for CameraNode {
    fn init(base: Base<Camera2D>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_enabled(true);
        self.base_mut().make_current();
    }

    fn process(&mut self, delta: f64) {

    }
}

#[godot_api]
impl CameraNode {}
