use std::{cell::RefCell, collections::HashMap, rc::Rc};

use common::game_world::GameWorld;
use godot::prelude::*;

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
    player_id: Option<u32>,

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
            player_id: None,
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

            self.network_client = Some(client.clone());
            let c = client.clone();
            godot_print!("Creating player's client...");
            let rt = tokio::runtime::Runtime::new().expect("err");
            match rt.block_on(NetworkClient::send_handshake()) {
                Ok(id) => {
                    self.player_id = Some(id);

                    // local player spawn
                    let mut local_player = self.player_scene.instantiate_as::<PlayerWrapper>();
                    local_player.bind_mut().set_id(id);
                    local_player.bind_mut().set_client_network(c);
                    self.base_mut()
                        .add_child(&local_player.clone().upcast::<Node>());

                    godot_print!("{id}");

                    self.players.insert(id, local_player);
                    godot_print!("Player connected to NetworkClient node");
                }
                _ => {}
            }
        } else {
            godot_error!("Could not find NetworkClient node");
        }
    }
}

#[godot_api]
impl World {
    pub fn on_snapshot_update(&mut self, world_wrapper: Gd<GameWorldWrapper>) {
        let world = world_wrapper.bind().game_world.clone();

        // Setup players
        for (id, player_data) in world.unwrap().players {
            if let Some(player) = self.players.get_mut(&id) {
                player.bind_mut().update_position(Vector3 {
                    x: player_data.x,
                    y: player_data.y,
                    z: player_data.rotation,
                });
            } else {
                godot_print!("new player");
                let mut player = self.player_scene.instantiate_as::<PlayerWrapper>();
                player.bind_mut().set_id(id);
                self.base_mut()
                    .add_child(&player.clone().upcast::<Node>());
                self.players.insert(id, player);
            }
        }

        // Setup bullets

        // Setup items / asteroids, EVERYTHING IN WORLD
    }
}
