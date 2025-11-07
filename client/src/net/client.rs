use core::panic;
use godot::{classes::Engine, prelude::*};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::thread;
use tokio::sync::mpsc;

use crate::net::NetworkClient;

#[derive(Serialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Clone)]
struct AuthResponse {
    id: u32,
}

#[derive(Clone)]
enum AuthResult {
    LoginOk(AuthResponse),
    RegisterOk(AuthResponse),
    Error(String),
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkAPI {
    base: Base<Node>,
    tx: mpsc::Sender<AuthResult>,
    rx: Option<mpsc::Receiver<AuthResult>>,
    last_message: Option<AuthResult>,
    server_address: String,
}

#[godot_api]
impl INode for NetworkAPI {
    fn init(base: Base<Node>) -> Self {
        let client = match Engine::singleton().get_singleton("NetworkClient") {
            None => {
                godot_error!("Failed to get singleton");
                panic!("wooh wooh");
            }
            Some(s) => s.try_cast::<NetworkClient>().unwrap(),
        };
        let server_address = client.bind().auth_server_address.clone();
        let (tx, rx) = mpsc::channel(8);
        Self {
            base,
            tx,
            rx: Some(rx),
            last_message: None,
            server_address,
        }
    }

    fn ready(&mut self) {}

    fn process(&mut self, _delta: f64) {
        // poll messages (non-blocking)
        if let Some(rx) = &mut self.rx {
            match rx.try_recv() {
                Ok(msg) => {
                    match &msg {
                        AuthResult::LoginOk(r) => {
                            self.base_mut().emit_signal("login_response_arrived", &[Variant::from(r.id)]);
                            godot_print!("Login success: {}", r.id)
                        }
                        AuthResult::RegisterOk(r) => {
                            godot_print!("Register success: {}", r.id)
                        }
                        AuthResult::Error(e) => godot_print!("Auth error: {}", e),
                    }
                    self.last_message = Some(msg);
                }
                Err(_) => {}
            }
        }
    }
}

#[godot_api]
impl NetworkAPI {
    #[signal]
    pub fn login_response_arrived(id: u32);

    #[signal]
    pub fn register_response_arrived(id: u32);

    #[signal]
    pub fn get_servers_response_arrived(id: u32);

    #[func]
    pub fn login(&self, username: GString, password: GString) {
        let tx = self.tx.clone();
        let username = username.to_string();
        let password = password.to_string();
        let server_address = format!("http://{}/login", self.server_address.clone());
        godot_print!("{username} {password} {server_address}");

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let client = Client::new();
                let payload = AuthRequest { username, password };

                let result = match client.post(server_address).json(&payload).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        match resp.json::<AuthResponse>().await {
                            Ok(r) => AuthResult::LoginOk(r),
                            Err(_) => AuthResult::Error("Invalid JSON".into()),
                        }
                    }
                    Ok(resp) => AuthResult::Error(format!("HTTP {}", resp.status())),
                    Err(err) => AuthResult::Error(err.to_string()),
                };

                let _ = tx.send(result).await;
            });
        });
    }

    #[func]
    pub fn register(&self, username: GString, password: GString) {
        let tx = self.tx.clone();
        let username = username.to_string();
        let password = password.to_string();
        let server_address = format!("http://{}/register", self.server_address.clone());

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let client = Client::new();
                let payload = AuthRequest { username, password };

                let result = match client.post(server_address).json(&payload).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        match resp.json::<AuthResponse>().await {
                            Ok(r) => AuthResult::RegisterOk(r),
                            Err(_) => AuthResult::Error("Invalid JSON".into()),
                        }
                    }
                    Ok(resp) => AuthResult::Error(format!("HTTP {}", resp.status())),
                    Err(err) => AuthResult::Error(err.to_string()),
                };

                let _ = tx.send(result).await;
            });
        });
    }
}
