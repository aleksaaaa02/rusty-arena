use godot::prelude::*;

use crate::entry::EntryNode;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainMenuNode {
    base: Base<Node2D>,
}


#[godot_api]
impl INode2D for MainMenuNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
        } 
    } 
}

#[godot_api]
impl MainMenuNode {

    #[func]
    pub fn new_game_pressed(&mut self) {
        let parent = match self.base().get_parent() {
            Some(p)  => p,
            None => { godot_error!("unable to find parent"); return; }
        };

        let mut entry = match parent.try_cast::<EntryNode>() {
           Err(_) => {
            godot_error!("Unable to cast to Entry node");
            return;
           } ,
           Ok(n) => n
        };

        entry.bind_mut().navigate_to_game_scene();
    }
    
}