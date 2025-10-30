use godot::prelude::*;




#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct AsteroidWrapper {
    base: Base<Node2D>,
    id: u32,
    x: f32,
    y: f32,
}


#[godot_api]
impl INode2D for AsteroidWrapper {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            id: 0,
            x: 0.0,
            y: 0.0,
        }
    }

    fn physics_process(&mut self, delta: f32) {
        let new_pos = Vector2::new(self.x, self.y);
        self.base_mut().set_position(new_pos);
    }

}

#[godot_api]
impl AsteroidWrapper {
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;

    }
    
}