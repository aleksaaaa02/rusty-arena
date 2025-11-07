pub mod async_runtime;
pub mod client;
pub mod packets;

use common::game_world::GameWorld;
use common::packet::InputAction;
use godot::prelude::*;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

use crate::game_world::GameWorldWrapper;
use crate::net::async_runtime::AsyncRuntime;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct NetworkClient {
    base: Base<Object>,
    socket: Option<Arc<UdpSocket>>,
    game_server_address_udp: String,
    game_server_address_tcp: String,
    controller_id: u32,
    pub auth_server_address: String,
    pub snapshot_rx: Option<UnboundedReceiver<GameWorld>>,
}

#[godot_api]
impl NetworkClient {
    #[signal]
    pub fn new_snapshot(world: Gd<GameWorldWrapper>);

    // #[func]
    pub fn start_listening(&mut self) -> Option<UnboundedReceiver<GameWorld>> {
        let Some(socket) = &self.socket else {
            godot_error!("Not listening");
            return None;
        };

        let listen_sock = socket.clone();
        let (tx, rx) = unbounded_channel();

        AsyncRuntime::spawn(async move {
            let mut buf = [0u8; 4096];
            let config = bincode::config::standard();
            loop {
                match listen_sock.recv(&mut buf).await {
                    Ok(len) => {
                        if let Ok((world, _)) =
                            bincode::decode_from_slice::<GameWorld, _>(&buf[..len], config)
                        {
                            if tx.send(world).is_err() {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        godot_error!("Failed to receive snapshot: {}", e);
                        break;
                    }
                }
            }
        });

        Some(rx)
    }

    // #[func]
    pub fn connect_to_server(&mut self) {
        let addr = &self.game_server_address_udp;
        let sock_future = async move {
            match UdpSocket::bind("0.0.0.0:0").await {
                Ok(sock) => {
                    if let Err(e) = sock.connect(addr).await {
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
        let socket = AsyncRuntime::block_on(sock_future);
        self.socket = socket;
    }

    #[func]
    pub fn send_input(&self, id: u32, seq: u32, action_code: u32) {
        let socket = self.socket.clone();
        let input = common::packet::PlayerInput {
            id,
            seq,
            action: InputAction::get_action_from_code(action_code),
        };

        let input_bytes = bincode::encode_to_vec(input, bincode::config::standard()).unwrap();
        AsyncRuntime::spawn(async move {
            let _ = socket.unwrap().send(&input_bytes).await;
        });
    }

    pub fn set_controller_id(&mut self, controller_id: u32) {
        self.controller_id = controller_id;
    }

    pub fn controller_id(&self) -> u32 {
        self.controller_id
    }

    pub fn set_config(&mut self, game_server_address_udp: String, game_server_address_tcp: String, auth_server_address: String) {
        self.auth_server_address = auth_server_address;
        self.game_server_address_udp = game_server_address_udp;
        self.game_server_address_tcp = game_server_address_tcp;
    }

    pub async fn send_handshake(&mut self) -> Result<u32, std::io::Error> {
        let auth_address = &self.game_server_address_tcp;
        let mut stream = TcpStream::connect(auth_address).await?;

        let id = self.controller_id;

        let request = self.controller_id.to_be_bytes();
        stream.write_all(&request).await?;

        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).await?;

        let player_id = u32::from_be_bytes(buffer);
        godot_print!("{player_id}");
        Ok(player_id)
    }

}
