use crate::camera::CameraNode;
use crate::net::NetworkClient;
use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerWrapper {
    #[base]
    base: Base<CharacterBody2D>,
    id: u32,
    #[export]
    network_path: NodePath,
    data: common::game_world::Player,
    network_client: Option<Gd<NetworkClient>>,
}

#[godot_api]
impl ICharacterBody2D for PlayerWrapper {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            base,
            id: 0,
            network_client: None,
            network_path: NodePath::from("NetworkClient"),
            data: common::game_world::Player {
                id: 0,
                hp: 100,
                rotation: 0.0,
                vx: 0.0,
                vy: 0.0,
                x: 0.0,
                y: 0.0,
                fire_rate_ms: 0,
                last_shot_ms: 0
            },
        }
    }

    fn ready(&mut self) {
        let node = self.base().get_node_as::<NetworkClient>(&self.network_path);
        if let client = node {
            godot_print!("Creating player's client...");
            self.network_client = Some(client.clone());
            godot_print!("Player connected to NetworkClient node");
        } else {
            godot_error!("Could not find NetworkCleint node");
        }
    }

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
                godot_print!("Goind left");
                client.bind().send_input(self.id, 1);
            }
        }

        if input.is_action_pressed("ui_right") {
            if let Some(client) = &self.network_client {
                godot_print!("Goind right");
                client.bind().send_input(self.id, 2);
            }
        }

        if input.is_action_pressed("ui_up") {
            if let Some(client) = &self.network_client {
                godot_print!("Goind forward");
                client.bind().send_input(self.id, 3);
            }
        }

        if input.is_action_pressed("ui_select") {
            if let Some(client) = &self.network_client {
                godot_print!("Pow pow");
                client.bind().send_input(self.id, 4);
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
        self.base_mut().emit_signal("health_updated", &[Variant::from(hp)]);
    }

    pub fn set_id(&mut self, id: u32) {
        self.data.id = id;
        self.id = id;
    }

    pub fn set_client_network(&mut self, network_client :Gd<NetworkClient>){
        self.network_client = Some(network_client);
    }

    pub fn spawn_camera(&mut self) {
        let cam = CameraNode::new_alloc();
        self.base_mut().add_child(&cam.clone().upcast::<CameraNode>());
    }     
}
