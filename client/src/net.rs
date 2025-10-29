use common::packet::PlayerInput;
use godot::classes::{INode, Node};
use godot::prelude::*;
use std::sync::{Arc};
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkClient {
    base: Base<Node>,
    runtime: Runtime,
    socket: Option<Arc<UdpSocket>>,
    server_addr: String,
    is_connected: bool,
    input_tx: Option<Sender<PlayerInput>>,
}

#[godot_api]
impl INode for NetworkClient {
    fn init(base: Base<Node>) -> Self {
        Self {
            base: base,
            runtime: Runtime::new().unwrap(),
            socket: None,
            server_addr: String::new(),
            input_tx: None,
            is_connected: false,
        }
    }

}

#[godot_api]
impl NetworkClient {
    #[func]
    pub fn connect_to_server(&mut self, server_ip: GString, server_port: i32) {
        let addr = format!("{}:{}", server_ip, server_port);
        let sock_future = async move {
            match UdpSocket::bind("0.0.0.0:0").await {
                Ok(sock) => {
                    if let Err(e) = sock.connect(&addr).await {
                        godot_error!("Failed to connect to server: {}", e);
                        return None;
                    }
                    Some(Arc::new(sock))
                }
                Err(_) => {
                    godot_error!("Oopsie");
                    return None;
                }
            }
        };

        let socket = self.runtime.block_on(sock_future);
        self.socket = socket;
    }

    #[func]
    pub fn send_input(&self, id: u32, action_code: u32) {
        godot_print!("SENDING INPUT");
        let socket = self.socket.clone();
        let input = common::packet::PlayerInput  {
            id: id,
            action: match action_code {
                1 => common::packet::InputAction::RotateLeft ,
                2 => common::packet::InputAction::RotateRight ,
                3 => common::packet::InputAction::Shoot ,
                4 => common::packet::InputAction::Thrust ,
                _ => common::packet::InputAction::Shoot 
            }
        };
            
        let input_bytes = bincode::encode_to_vec(input, bincode::config::standard()).unwrap();
        self.runtime.spawn(async move {
            let _ = socket.unwrap().send(&input_bytes).await;
        });
    }
}

