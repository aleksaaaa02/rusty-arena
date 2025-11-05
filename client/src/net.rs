// pub mod client;

use common::game_world::GameWorld;
use common::packet::PlayerInput;
use godot::classes::{ConfigFile, INode, Node};
use godot::prelude::*;
use godot::global::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Sender, UnboundedReceiver, unbounded_channel};

use crate::game_world::GameWorldWrapper;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkClient {
    base: Base<Node>,
    runtime: Runtime,
    socket: Option<Arc<UdpSocket>>,
    server_addr: String,
    is_connected: bool,
    input_tx: Option<Sender<PlayerInput>>,
    snapshot_rx: Option<UnboundedReceiver<GameWorld>>,
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
            snapshot_rx: None,
        }
    }

    fn ready(&mut self) {
        let mut conf = ConfigFile::new_gd();
        let err = conf.load("res://server.cfg");

        if err == Error::OK {
            self.server_addr = conf.get_value("GameServer", "address").to_string();
        }
        self.server_addr = String::from("127.0.0.0:8080");


    }

    fn process(&mut self, delta: f64) {
        if let Some(mut rx) = self.snapshot_rx.take() {
            let mut worlds = Vec::new();

            while let Ok(world) = rx.try_recv() {
                worlds.push(world);
            }

            self.snapshot_rx = Some(rx);

            for world in worlds {
                let world_wrapped = GameWorldWrapper::from_game_world(world);
                self.base_mut()
                    .emit_signal("new_snapshot", &[world_wrapped.to_variant()]);
            }
        }
    }
}

#[godot_api]
impl NetworkClient {
    #[signal]
    pub fn new_snapshot(world: Gd<GameWorldWrapper>);

    #[func]
    fn start_listening(&mut self) {
        let Some(socket) = &self.socket else {
            godot_error!("Not listening");
            return;
        };

        let listen_sock = socket.clone();
        let (tx, rx) = unbounded_channel();
        self.snapshot_rx = Some(rx);

        self.runtime.spawn(async move {
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
    }

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
        godot_print!("SENDING INPUT {id} {action_code}");
        let socket = self.socket.clone();
        let input = common::packet::PlayerInput {
            id: id,
            action: match action_code {
                1 => common::packet::InputAction::RotateLeft,
                2 => common::packet::InputAction::RotateRight,
                3 => common::packet::InputAction::Thrust,
                4 => common::packet::InputAction::Shoot,
                5 => common::packet::InputAction::Hello,
                _ => common::packet::InputAction::Hello
            },
        };

        let input_bytes = bincode::encode_to_vec(input, bincode::config::standard()).unwrap();
        self.runtime.spawn(async move {
            let _ = socket.unwrap().send(&input_bytes).await;
        });
    }

    pub async fn send_handshake() -> Result<u32, std::io::Error> {
        let mut stream = TcpStream::connect("127.0.0.1:8081").await?;

        godot_print!("hewo");
        let request = b"HELLO_UWU";
        stream.write_all(request).await?;

        godot_print!("hewo_poslah");
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).await?;

        godot_print!("hewo_pwocitah");
        let player_id = u32::from_be_bytes(buffer);
        godot_print!("{player_id}");
        Ok(player_id)
    }
}
