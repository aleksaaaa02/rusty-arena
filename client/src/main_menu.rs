use godot::{classes::LineEdit, prelude::*};

use crate::{entry::EntryNode, net::{async_runtime::AsyncRuntime, client::NetworkAPI}};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainMenuNode {
    base: Base<Node2D>,
    // username: Option<Gd<LineEdit>>,
    // password: Option<Gd<LineEdit>>,
}

#[godot_api]
impl INode2D for MainMenuNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            // username: None,
            // password: None,
        }
    }

    fn ready(&mut self) {
        // self.username = Some(
        //     self.base()
        //         .get_node_as::<LineEdit>("CanvasLayer/WelcomeScreen/Username"),
        // );

        // self.password = Some(
        //     self.base()
        //         .get_node_as::<LineEdit>("CanvasLayer/WelcomeScreen/Password"),
        // );
    }
}

#[godot_api]
impl MainMenuNode {

    #[func]
    pub fn on_login_success_happend(&mut self, id: u32) {
        godot_print!("Horray!");
    }

    #[func]
    pub fn new_game_pressed(&mut self) {
        // let parent = match self.base().get_parent() {
        //     Some(p) => p,
        //     None => {
        //         godot_error!("unable to find parent");
        //         return;
        //     }
        // };

        // let mut entry = match parent.try_cast::<EntryNode>() {
        //     Err(_) => {
        //         godot_error!("Unable to cast to Entry node");
        //         return;
        //     }
        //     Ok(n) => n,
        // };
        // entry.bind_mut().navigate_to_game_scene();
        let username_node = self
            .base()
            .get_node_as::<LineEdit>("CanvasLayer/WelcomeScreen/Username");

        let password_node = self
            .base()
            .get_node_as::<LineEdit>("CanvasLayer/WelcomeScreen/Password");

        let username = username_node.get_text();
        let password = password_node.get_text();

        let network_api = self
            .base()
            .get_node_as::<NetworkAPI>("NetworkAPI");

        network_api.bind().login(username, password);
    }

    #[func]
    pub fn on_register_click(&mut self) {
        let username_node = self
            .base()
            .get_node_as::<LineEdit>("CanvasLayer/WelcomeScreen/Username");

        let password_node = self
            .base()
            .get_node_as::<LineEdit>("CanvasLayer/WelcomeScreen/Password");

        let username = username_node.get_text();
        let password = password_node.get_text();

        let network_api = self
            .base()
            .get_node_as::<NetworkAPI>("NetworkAPI");

        network_api.bind().register(username, password);
    }

    #[func]
    pub fn on_register_success(&mut self, user_id: u32) {}

    #[func]
    pub fn on_login_success(&mut self, user_id: u32) {}
}
