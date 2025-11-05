use common::game_world::GameWorld;

pub struct GameState {
    world_state: GameWorld
}

impl GameState {
    
    pub fn new(game_world: GameWorld) -> Self {
        Self {
            world_state: game_world
        }
    }
}
