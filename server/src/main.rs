use bincode::config;
use common::{game_world::GameWorld, packet::PlayerInput};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::{io, sync::Arc};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::Instant;

const TICK_DURATION: u64 = 16;

#[tokio::main]
async fn main() -> io::Result<()> {
    let bind = "0.0.0.0:8080";

    let socket = Arc::new(UdpSocket::bind(bind).await?);

    let (input_tx, input_rx) = mpsc::channel::<(SocketAddr, PlayerInput)>(1024);
    let (snapshot_tx, mut snapshot_rx) = mpsc::channel::<GameWorld>(1024);

    let clients = Arc::new(Mutex::new(HashSet::<SocketAddr>::new()));

    let input_tx_listener = input_tx.clone();

    {
        let socket_listener = socket.clone();
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
        let socket_output = socket.clone();
        let clients = clients.clone();

        // networking output
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

    game_loop(input_rx, snapshot_tx).await;

    Ok(())
}

async fn game_loop(
    mut input_rx: Receiver<(SocketAddr, PlayerInput)>,
    snapshot_tx: Sender<GameWorld>,
) {
    let tick_duration = std::time::Duration::from_millis(TICK_DURATION);
    let mut world = GameWorld::new();
    let mut addr_to_id = HashMap::new();

    loop {
        let start = Instant::now();

        while let Ok((addr, input)) = input_rx.try_recv() {
            let id = *addr_to_id
                .entry(addr)
                .or_insert_with(|| world.add_player(input.id));
            world.apply_input(input.id, &input);
            println!("{:?}", input);
        }

        let _ = snapshot_tx.send(world.clone()).await;

        world.update();

        let elapsed = start.elapsed();
        if elapsed < tick_duration {
            tokio::time::sleep(tick_duration - elapsed).await;
        }
        println!("brm, next ->");
    }
}
