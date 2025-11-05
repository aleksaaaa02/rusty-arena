use crate::camera::CameraNode;
use crate::net::NetworkClient;
use common::player::Player;
use godot::classes::{CharacterBody2D, Engine, ICharacterBody2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerWrapper {
    #[base]
    base: Base<CharacterBody2D>,
    id: u32,
    data: Player,
    network_client: Option<Gd<NetworkClient>>,
}

#[godot_api]
impl ICharacterBody2D for PlayerWrapper {
    fn init(base: Base<CharacterBody2D>) -> Self {

        Self {
            base,
            id: 0,
            network_client: None,
            data: Player {
                id: 0,
                hp: 100,
                rotation: 0.0,
                vx: 0.0,
                vy: 0.0,
                x: 0.0,
                y: 0.0,
                fire_rate_ms: 0,
                last_shot_ms: 0,
            },
        }
    }

    fn ready(&mut self) {}

    fn physics_process(&mut self, delta: f64) {
        {
            let new_post = Vector2 {
                x: self.data.x,
                y: self.data.y,
            };
            let rotation = self.data.rotation;
            self.base_mut().set_global_position(new_post);
            self.base_mut().set_rotation(rotation);
        }

        let input = Input::singleton();

        if input.is_action_pressed("ui_left") {
            if let Some(client) = &self.network_client {
                client.bind().send_input(1);
            }
        }

        if input.is_action_pressed("ui_right") {
            if let Some(client) = &self.network_client {
                client.bind().send_input(2);
            }
        }

        if input.is_action_pressed("ui_up") {
            if let Some(client) = &self.network_client {
                client.bind().send_input(3);
            }
        }

        if input.is_action_pressed("ui_select") {
            if let Some(client) = &self.network_client {
                client.bind().send_input(4);
            }
        }
    }
}

#[godot_api]
impl PlayerWrapper {
    #[signal]
    pub fn health_updated(current_hp: u16);

    pub fn update_position(&mut self, new_position: Vector3) {
        self.data.x = new_position.x;
        self.data.y = new_position.y;
        self.data.rotation = new_position.z;
    }

    pub fn update_health(&mut self, hp: u16) {
        self.data.hp = hp;
        self.base_mut()
            .emit_signal("health_updated", &[Variant::from(hp)]);
    }

    pub fn set_id(&mut self, id: u32) {
        self.data.id = id;
        self.id = id;
    }

    pub fn spawn_camera(&mut self) {
        let cam = CameraNode::new_alloc();
        self.base_mut()
            .add_child(&cam.clone().upcast::<CameraNode>());

        let client = match Engine::singleton().get_singleton("NetworkClient") {
            None => {
                godot_error!("Failed to get singleton");
                return;
            }
            Some(s) => s.try_cast::<NetworkClient>().unwrap(),
        };

        self.network_client = Some(client.clone());
    }
}
