use common::game_world::GameWorld;
use godot::classes::{IRefCounted, RefCounted};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GameWorldWrapper {
    pub base: Base<RefCounted>,
    pub game_world: Option<GameWorld>,
}

#[godot_api]
impl IRefCounted for GameWorldWrapper {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            game_world: None
        }
    }
}

#[godot_api]
impl GameWorldWrapper {
    pub fn from_game_world(game_world: GameWorld) -> Gd<Self> {
        let mut new = Self::new_gd();
        new.bind_mut().game_world = Some(game_world);
        new

    }
}
