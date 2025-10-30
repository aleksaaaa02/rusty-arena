use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

pub mod player;
pub mod net;
pub mod game_world;
pub mod bullet;
pub mod world;
pub mod camera;
pub mod ui_layer;
pub mod asteroids;