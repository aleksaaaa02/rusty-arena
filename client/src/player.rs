use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::prelude::*;
use crate::net::NetworkClient;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    #[base]
    base: Base<CharacterBody2D>,
    id: u32,
    #[export]
    network_path: NodePath,
    data: common::game_world::Player,
    network_client: Option<Gd<NetworkClient>>,
}

#[godot_api]
impl ICharacterBody2D for Player {
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
            },
        }
    }

    fn ready(&mut self) {
        let node = self.base().get_node_as::<NetworkClient>(&self.network_path);
        if let client = node {
            self.network_client = Some(client);
            godot_print!("Player connected to NetworkClient node");
        } else {
            godot_error!("Could not find NetworkCleint node");
        }
    }

    fn physics_process(&mut self, delta: f64) {

        let input = Input::singleton();
        
        if input.is_action_pressed("ui_left") {
            godot_print!("Goind left");
            if let Some(client) = &self.network_client {
                client.bind().send_input(self.id, 1);
            }
        }
        
        if input.is_action_pressed("ui_right") {
            godot_print!("Goind right");
        }

        if input.is_action_pressed("ui_up") {
            godot_print!("Goind forward");
        }
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {}

    #[signal]
    fn speed_increased();
}
