use crate::game_world::GameWorldWrapper;
use crate::net::NetworkClient;
use crate::player;
use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct PlayerWrapper {
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
            },
        }
    }

    fn ready(&mut self) {
        let node = self.base().get_node_as::<NetworkClient>(&self.network_path);
        if let client = node {
            godot_print!("Subscribing player to network...");
            client
                .signals()
                .new_snapshot()
                .connect_other(self, |this, _world| {
                    this.on_snapshot_update(_world);
                });
            godot_print!("Creating player's client...");
            self.network_client = Some(client.clone());
            godot_print!("Player connected to NetworkClient node");
        } else {
            godot_error!("Could not find NetworkCleint node");
        }
    }

    fn physics_process(&mut self, delta: f64) {
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
    }
}

#[godot_api]
impl PlayerWrapper {

    #[func]
    pub fn on_snapshot_update(&mut self, world_wrapper: Gd<GameWorldWrapper>) {
        let world = &world_wrapper.bind().game_world;

        godot_print!("{:?}", world);

        if let Some(player_data) = world.players.get(&self.id) {
            self.base_mut().set_global_position(Vector2 { x: player_data.x, y: player_data.y });

            self.base_mut().set_rotation(player_data.rotation);

            self.data.vx = player_data.vx;
            self.data.vy = player_data.vy;
        }
    }
}
