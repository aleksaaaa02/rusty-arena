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
pub mod health_bar;

use godot::global::Error;
use godot::{
    classes::{ConfigFile, Engine},
    prelude::*,
};

use crate::net::NetworkClient;
use crate::net::async_runtime::AsyncRuntime;

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
            let game_server_udp = conf.get_value("GameServer", "address_udp").to_string();
            let game_server_tcp = conf.get_value("GameServer", "address_tcp").to_string();
            let auth_server = conf.get_value("AuthServer", "address").to_string();
            // godot_print!("Hello, GameServer: {game_server} | AuthServer: {auth_server}");
            instance.bind_mut().set_config(game_server_udp, game_server_tcp, auth_server);
        } else {
            let game_server_udp = String::from("127.0.0.1:8080");
            let game_server_tcp = String::from("127.0.0.1:8081");
            let auth_server = String::from("127.0.0.1:3000");
            // godot_print!("Going to defauls, GameServer: {game_server} | AuthServer: {auth_server}");
            instance.bind_mut().set_config(game_server_udp, game_server_tcp, auth_server);
        }

        Engine::singleton().register_singleton("NetworkClient", &instance);
        Engine::singleton().register_singleton(AsyncRuntime::SINGLETON, &AsyncRuntime::new_alloc());
    }

    fn on_level_deinit(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        let mut engine = Engine::singleton();
        match engine.get_singleton("NetworkClient") {
            Some(s) => {
                engine.unregister_singleton("NetworkClient");
                s.free();
            }
            None => godot_error!("Failed to get singleton"),
        }
        match engine.get_singleton(AsyncRuntime::SINGLETON) {
            Some(async_singleton) => {
                engine.unregister_singleton(AsyncRuntime::SINGLETON);
                async_singleton.free();
            }
            None => {
                godot_warn!(
                    "Failed to find & free singleton -> {}",
                    AsyncRuntime::SINGLETON
                );
            }
        }
    }
}
