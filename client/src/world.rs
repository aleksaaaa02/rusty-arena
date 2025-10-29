use std::collections::HashMap;

use common::game_world::GameWorld;
use godot::{classes::CharacterBody2D, prelude::*};

use crate::{
    bullet::BulletNode, game_world::GameWorldWrapper, net::NetworkClient, player::PlayerWrapper,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World {
    base: Base<Node2D>,
    players: HashMap<u32, Gd<PlayerWrapper>>,
    bullets: HashMap<u32, Gd<BulletNode>>,
    last_snapshot: GameWorld,
    network_client: Option<Gd<NetworkClient>>,
    player_scene: Gd<PackedScene>,
    #[export]
    network_path: NodePath,
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            players: HashMap::new(),
            bullets: HashMap::new(),
            last_snapshot: GameWorld {
                players: HashMap::new(),
                bullets: Vec::new(),
                asteroids: Vec::new(),
                width: 800.0,
                height: 800.0,
            },
            player_scene: load("res://player.tscn"),
            network_client: None,
            network_path: NodePath::from("NetworkClient"),
        }
    }

    fn process(&mut self, delta: f64) {}

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

            let mut local_player = self.player_scene.instantiate_as::<PlayerWrapper>();
            local_player.bind_mut().set_id(0);
            local_player.bind_mut().set_client_network(client.clone());
            self.base_mut().add_child(&local_player.clone().upcast::<Node>());
            self.players.insert(0, local_player);
        } else {
            godot_error!("Could not find NetworkClient node");
        }
    }
}

#[godot_api]
impl World {
    pub fn on_snapshot_update(&mut self, world_wrapper: Gd<GameWorldWrapper>) {
        // self.last_snapshot = world_wrapper.bind().game_world.clone();
        let world = world_wrapper.bind().game_world.clone();
        godot_print!("{:?}", world);

        // Setup players
        for (id, player_data) in world.unwrap().players {
            if let Some(player) = self.players.get_mut(&id) {
                player.bind_mut().update_position(Vector3 {
                    x: player_data.x,
                    y: player_data.y,
                    z: player_data.rotation
                });
            }
        }

        // Setup bullets

        // Setup items / asteroids, EVERYTHING IN WORLD
    }
}
