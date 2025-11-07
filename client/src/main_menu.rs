use godot::{classes::{Engine, LineEdit}, prelude::*};

use crate::{entry::EntryNode, net::{NetworkClient, async_runtime::AsyncRuntime, client::NetworkAPI}};

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
    pub fn on_login_click(&mut self) {
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
    pub fn on_get_servers_success(&mut self, server_list: Array<GString>) {

    }

    #[func]
    pub fn on_login_success(&mut self, id: u32) {
        let network_api = self
            .base()
            .get_node_as::<NetworkAPI>("NetworkAPI");

        let mut client = match Engine::singleton().get_singleton("NetworkClient") {
            None => {
                godot_error!("Failed to get singleton");
                return;
            }
            Some(s) => s.try_cast::<NetworkClient>().expect("OVDE SMO PUKLI"),
        };
        client.bind_mut().set_controller_id(id);

        let parent = match self.base().get_parent() {
            Some(p) => p,
            None => {
                godot_error!("unable to find parent");
                return;
            }
        };

        let mut entry = match parent.try_cast::<EntryNode>() {
            Err(_) => {
                godot_error!("Unable to cast to Entry node");
                return;
            }
            Ok(n) => n,
        };

        entry.bind_mut().navigate_to_game_scene();

        // network_api.bind().get_servers();
    }
}
