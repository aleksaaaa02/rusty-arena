pub mod asteroids;
pub mod bullet;
pub mod camera;
pub mod entry;
pub mod game_world;
pub mod main_menu;
pub mod net;
pub mod player;
pub mod ui_layer;
pub mod world;

use godot::global::Error;
use godot::{
    classes::{ConfigFile, Engine},
    prelude::*,
};

use crate::net::NetworkClient;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        let mut instance = NetworkClient::new_alloc();

        let mut conf = ConfigFile::new_gd();
        let err = conf.load("res://server.cfg");

        if err == Error::OK {
            let game_server = conf.get_value("GameServer", "address").to_string();
            let auth_server = conf.get_value("AuthServer", "address").to_string();
            godot_print!("Hello, GameServer: {game_server} | AuthServer: {auth_server}");
            instance.bind_mut().set_config(game_server, auth_server);
        } else {
            let game_server = String::from("127.0.0.0:8080");
            let auth_server = String::from("127.0.0.0:8081");
            instance.bind_mut().set_config(game_server, auth_server);
        }

        Engine::singleton().register_singleton("NetworkClient", &instance);
    }

    fn on_level_deinit(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        let mut engine = Engine::singleton();
        match Engine::singleton().get_singleton("NetworkClient") {
            None => godot_error!("Failed to get singleton"),
            Some(s) => {
                engine.unregister_singleton("NetworkClient");
                s.free();
            }
        }
    }
}
