use bincode::config;
use common::{game_world::GameWorld, packet::PlayerInput};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::{io, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::Instant;

const TICK_DURATION: u64 = 16;

#[tokio::main]
async fn main() -> io::Result<()> {
    let bind = "0.0.0.0:8080";
    let bind_tcp = "0.0.0.0:8081";
    let socket = Arc::new(UdpSocket::bind(bind).await?);
    let tcp_socket = Arc::new(TcpListener::bind(bind_tcp).await?);

    let (input_tx, input_rx) = mpsc::channel::<(SocketAddr, PlayerInput)>(1024);
    let (snapshot_tx, mut snapshot_rx) = mpsc::channel::<GameWorld>(1024);
    let (input_tcp_tx, input_tcp_rx) = mpsc::channel::<(SocketAddr, u32)>(128);

    let clients = Arc::new(Mutex::new(HashSet::<SocketAddr>::new()));
    let id_counter = Arc::new(AtomicU32::new(0));

    {
        // TCP/Auth -> ovo kasnije ce biti posebna aplikacija
        let id_counter = id_counter.clone();
        let input_tcp_tx = input_tcp_tx.clone();
        let tcp_socket = tcp_socket.clone();
        tokio::spawn(async move {
            let mut buf = [0u8; 64];

            loop {
                if let Ok((mut stream, addr)) = tcp_socket.accept().await {
                    let player_id =
                        id_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) as u32;

                    let n = stream.read(&mut buf).await.unwrap();
                    let req = String::from_utf8_lossy(&buf[..n]);

                    println!("{req}");

                    if req.trim() == "HELLO_UWU" {
                        println!("HELLO_UWU_I_TEBI")
                    }

                    let _ = stream.write_all(&player_id.to_be_bytes()).await;
                    let _ = stream.flush().await;

                    if let Err(e) = input_tcp_tx.send((addr, player_id)).await {
                        eprintln!("Failed to send input {e}");
                    }

                    println!("Assigned player_id={} to {}", player_id, addr);
                }
            }
        });
    }

    {
        // Task taking UDP load and sending command to game loop to update game state
        let socket_listener = socket.clone();
        let input_tx_listener = input_tx.clone();
        let clients = clients.clone();
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let config = config::standard();
            loop {
                let (len, addr) = socket_listener.recv_from(&mut buf).await.unwrap();

                {
                    clients.lock().await.insert(addr);
                }

                if let Ok((input, _)) =
                    bincode::decode_from_slice::<PlayerInput, _>(&buf[..len], config)
                {
                    if let Err(e) = input_tx_listener.send((addr, input)).await {
                        eprintln!("Failed to send input {e}");
                    }
                }
            }
        });
    }

    {
        // Task broadcasting world state
        let socket_output = socket.clone();
        let clients = clients.clone();
        tokio::spawn(async move {
            let config = config::standard();
            while let Some(world) = snapshot_rx.recv().await {
                let data = bincode::encode_to_vec(&world, config).unwrap();
                let clients = clients.lock().await.clone();
                for addr in clients {
                    if let Err(e) = socket_output.send_to(&data, addr).await {
                        eprintln!("Failed to broadcast data to {addr}: {e}");
                    }
                }
            }
        });
    }

    game_loop(input_rx, input_tcp_rx, snapshot_tx).await;

    Ok(())
}

async fn game_loop(
    mut input_rx: Receiver<(SocketAddr, PlayerInput)>,
    mut input_tcp: Receiver<(SocketAddr, u32)>,
    snapshot_tx: Sender<GameWorld>,
) {
    let tick_duration = std::time::Duration::from_millis(TICK_DURATION);
    let mut world = GameWorld::new();
    // let mut addr_to_id: HashMap<SocketAddr, u32> = HashMap::new();

    loop {
        let start = Instant::now();

        while let Ok((_, player_id)) = input_tcp.try_recv() {
            println!("new player");
            world.add_player(player_id);
        }

        while let Ok((_addr, input)) = input_rx.try_recv() {
            let id = input.id;
            world.apply_input(id, &input);
            println!("{:?}", input);
        }

        let _ = snapshot_tx.send(world.clone()).await;

        world.update();

        let elapsed = start.elapsed();
        if elapsed < tick_duration {
            tokio::time::sleep(tick_duration - elapsed).await;
        }
    }
}
