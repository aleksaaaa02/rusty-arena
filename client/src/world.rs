use std::collections::{HashMap, HashSet};

use common::game_world::GameWorld;
use godot::prelude::*;

use crate::{
    asteroids::AsteroidWrapper, bullet::BulletNode, game_world::GameWorldWrapper,
    net::NetworkClient, player::PlayerWrapper, ui_layer::UiLayer,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World {
    base: Base<Node2D>,
    players: HashMap<u32, Gd<PlayerWrapper>>,
    bullets: HashMap<u32, Gd<BulletNode>>,
    asteroids: HashMap<u32, Gd<AsteroidWrapper>>,
    last_snapshot: GameWorld,

    network_client: Option<Gd<NetworkClient>>,
    player_id: Option<u32>,
    
    player_scene: Gd<PackedScene>,
    asteroid_scene: Gd<PackedScene>,
    bullet_scene: Gd<PackedScene>,

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
            asteroids: HashMap::new(),
            last_snapshot: GameWorld {
                players: HashMap::new(),
                bullets: Vec::new(),
                asteroids: Vec::new(),
                width: 800.0,
                height: 800.0,
                bullet_id_counter: 0,
                asteroid_id_counter: 0,
                last_spawn_asteroid: 0,
            },
            player_id: None,
            player_scene: load("res://player.tscn"),
            bullet_scene: load("res://bullet.tscn"),
            asteroid_scene: load("res://asteroid.tscn"),
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
                    local_player.bind_mut().spawn_camera();
                    self.base_mut()
                        .add_child(&local_player.clone().upcast::<Node>());

                    if let mut ui_node = self.base().get_node_as::<UiLayer>("../UI") {
                        ui_node.bind_mut().connect_to_player(&local_player);
                    }
                    // Connect to UI - Health -> ovo treba srediti

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

    #[func]
    pub fn on_snapshot_update(&mut self, world_wrapper: Gd<GameWorldWrapper>) {
        let world = world_wrapper.bind().game_world.clone();

        // Setup players
        for (id, player_data) in world.clone().unwrap().players {
            if let Some(player) = self.players.get_mut(&id) {
                player.bind_mut().update_position(Vector3 {
                    x: player_data.x,
                    y: player_data.y,
                    z: player_data.rotation,
                });
                player.bind_mut().update_health(player_data.hp);
            } else {
                godot_print!("new player");
                let mut player = self.player_scene.instantiate_as::<PlayerWrapper>();
                player.bind_mut().set_id(id);
                self.base_mut().add_child(&player.clone().upcast::<Node>());
                self.players.insert(id, player);
            }
        }

        let server_bullet_ids: HashSet<u32> = world
            .clone()
            .unwrap()
            .bullets
            .iter()
            .map(|b| b.id)
            .collect();

        let bullets_to_remove: Vec<u32> = self
            .bullets
            .keys()
            .filter(|id| !server_bullet_ids.contains(id))
            .cloned()
            .collect();

        for id in bullets_to_remove {
            if let Some(bullet_node) = self.bullets.remove(&id) {
                self.base_mut()
                    .remove_child(&bullet_node.clone().upcast::<Node>());
            }
        }

        for bullet_data in world.clone().unwrap().bullets {
            if let Some(bullet) = self.bullets.get_mut(&bullet_data.id) {
                bullet.bind_mut().update_position(Vector2 {
                    x: bullet_data.x,
                    y: bullet_data.y,
                });
            } else {
                godot_print!("new bullet");
                let mut new_bullet = self.bullet_scene.instantiate_as::<BulletNode>();

                new_bullet.bind_mut().update_position(Vector2 {
                    x: bullet_data.x,
                    y: bullet_data.y,
                });
                new_bullet.bind_mut().set_id(bullet_data.id);
                self.base_mut()
                    .add_child(&new_bullet.clone().upcast::<Node>());

                self.bullets.insert(bullet_data.id, new_bullet);
            }
        }

        // Setup items / asteroids, EVERYTHING IN WORLD
        let server_asteroids_ids: HashSet<u32> = world
            .clone()
            .unwrap()
            .asteroids
            .iter()
            .map(|b| b.id)
            .collect();

        let asteroids_to_remove: Vec<u32> = self
            .asteroids
            .keys()
            .filter(|id| !server_asteroids_ids.contains(id))
            .cloned()
            .collect();

        for id in asteroids_to_remove {
            if let Some(asteroid) = self.asteroids.remove(&id) {
                self.base_mut()
                    .remove_child(&asteroid.clone().upcast::<Node>());
            }
        }

        for asteroid_data in world.clone().unwrap().asteroids {
            if let Some(asteroid) = self.asteroids.get_mut(&asteroid_data.id) {
                asteroid
                    .bind_mut()
                    .update_position(asteroid_data.x, asteroid_data.y);
            } else {
                godot_print!("new asteroid");
                let mut asteroid_node = self.asteroid_scene.instantiate_as::<AsteroidWrapper>();

                asteroid_node
                    .bind_mut()
                    .update_position(asteroid_data.x, asteroid_data.y);

                asteroid_node.bind_mut().set_id(asteroid_data.id);
                self.base_mut()
                    .add_child(&asteroid_node.clone().upcast::<Node>());

                self.asteroids.insert(asteroid_data.id, asteroid_node);
            }
        }
    }
}
