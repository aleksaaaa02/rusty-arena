use tokio::net::UdpSocket;
use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TestState {
    x: f32,
    y: f32,
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1024];
    let mut clients = HashMap::new();

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        let msg = String::from_utf8_lossy(&buf[..len]);

        clients.entry(addr).or_insert_with(|| {
            println!("New clinet: {}", addr);
            "Hello! world"
        });
        
        println!("{} says {}", addr, msg);

        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);

        for (&client_addr, _) in &clients {
            if client_addr != addr {
                sock.send_to(msg.as_bytes(), client_addr).await?;
            }
        }

    }
}