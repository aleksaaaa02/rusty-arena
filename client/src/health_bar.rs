use godot::{
    classes::{IProgressBar, ProgressBar},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=ProgressBar)]
pub struct HealthBar {
    base: Base<ProgressBar>,
}

#[godot_api]
impl IProgressBar for HealthBar { 

    fn physics_process(&mut self, delta: f64) {
        self.base_mut().set_rotation_degrees(0.0);
    }

    fn init(base: Base<ProgressBar>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl HealthBar {

    #[func]
    pub fn on_health_updated(&mut self, current_hp: u16) {
        self.base_mut().set_value(current_hp.into());
    }
}
