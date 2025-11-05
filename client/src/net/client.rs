use std::sync::Arc;
use common::game_world::GameWorld;
use common::packet::PlayerInput;
use tokio::net::{TcpStream, UdpSocket};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};

pub struct NetworkService {
    socket: Arc<UdpSocket>
}

impl NetworkService {
    pub async fn authenticate(server_ip: &str, port: u16) -> Result<u32, std::io::Error> {
        let mut stream = TcpStream::connect((server_ip, port)).await?;
        stream.write_all(b"HELLO_UWU").await?;

        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).await?;
        Ok(u32::from_be_bytes(buf))
    }

    pub async fn connect(server_ip: &str, port: u16)
        -> Result<(Self, UnboundedReceiver<GameWorld>), std::io::Error>
    {
        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
        socket.connect((server_ip, port)).await?;

        let (tx, rx) = unbounded_channel::<GameWorld>();

        let sock = socket.clone();
        tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            let config = bincode::config::standard();
            loop {
                if let Ok(len) = sock.recv(&mut buf).await {
                    if let Ok((world, _)) = bincode::decode_from_slice::<GameWorld, _>(&buf[..len], config) {
                        if tx.send(world).is_err() {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        });

        Ok((Self { socket }, rx))
    }

    pub async fn send_input(&self, input: PlayerInput) {
        let bytes = bincode::encode_to_vec(input, bincode::config::standard()).unwrap();
        let _ = self.socket.send(&bytes).await;
    }

}
