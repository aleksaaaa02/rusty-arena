use crate::camera::CameraNode;
use crate::net::NetworkClient;
use common::packet::InputAction;
use common::player::Player;
use godot::classes::{CharacterBody2D, Engine, ICharacterBody2D, Input, Sprite2D};
use godot::prelude::*;

#[derive(Clone)]
struct LocalInput {
    seq: u32,
    action: common::packet::InputAction,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerWrapper {
    #[base]
    base: Base<CharacterBody2D>,
    controller_id: Option<u32>,
    data: Player,
    sprite: Option<Gd<Sprite2D>>,
    input_seq: u32,
    pending_inputs: Vec<LocalInput>,
    network_client: Option<Gd<NetworkClient>>,
}

#[godot_api]
impl ICharacterBody2D for PlayerWrapper {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            controller_id: None,
            network_client: None,
            sprite: None,
            data: Player {
                id: 0,
                hp: 100,
                rotation: 0.0,
                vx: 0.0,
                vy: 0.0,
                x: 0.0,
                y: 0.0,
                fire_rate_ms: 0,
                last_shot_ms: 0,
                last_processed_input_seq: 0,
            },
            pending_inputs: Vec::new(),
            input_seq: 1,
        }
    }

    fn ready(&mut self) {
        let sprite = self.base_mut().get_child(0);
        match sprite {
            Some(s) => {
                if let Ok(sp) = s.try_cast::<Sprite2D>() {
                    self.sprite = Some(sp);
                }
            }
            None => {}
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if let Some(id) = self.controller_id {
            let mut new_inputs: Vec<InputAction> = Vec::new();

            let input = Input::singleton();

            if input.is_action_pressed("ui_left") {
                new_inputs.push(InputAction::RotateLeft);
            }

            if input.is_action_pressed("ui_right") {
                new_inputs.push(InputAction::RotateRight);
            }

            if input.is_action_pressed("ui_up") {
                new_inputs.push(InputAction::Thrust);
            }

            if input.is_action_pressed("ui_select") {
                new_inputs.push(InputAction::Shoot);
            }

            for action in new_inputs {
                let seq = self.input_seq;
                self.input_seq += 1;

                let local_input = LocalInput {
                    seq,
                    action: action.clone(),
                };

                self.pending_inputs.push(local_input.clone());

                if let Some(client) = &self.network_client {
                    client
                        .bind()
                        .send_input(id, seq, action.get_input_code_from_action());
                }

                self.apply_local_input(action, delta);
            }
        }
        let new_post = Vector2 {
            x: self.data.x,
            y: self.data.y,
        };
        self.base_mut().set_global_position(new_post);

        let rotation = self.data.rotation;
        if let Some(sprite) = &mut self.sprite {
            sprite.set_rotation(rotation);
        }
    }
}

#[godot_api]
impl PlayerWrapper {
    #[signal]
    pub fn health_updated(current_hp: u16);

    pub fn move_player(&mut self, delta: f64) {
        self.data.x += self.data.vx * delta as f32;
        self.data.y += self.data.vy * delta as f32;

        self.data.vx *= 0.98;
        self.data.vy *= 0.98;
    }

    pub fn update_position(&mut self, new_position: Vector3) {
        self.data.x = new_position.x;
        self.data.y = new_position.y;
        self.data.rotation = new_position.z;
    }

    pub fn update_health(&mut self, hp: u16) {
        self.data.hp = hp;
        self.base_mut()
            .emit_signal("health_updated", &[Variant::from(hp)]);
    }

    pub fn set_id(&mut self, id: u32) {
        self.data.id = id;
    }

    pub fn set_controller_id(&mut self, id: u32) {
        self.controller_id = Some(id);
    }

    pub fn spawn_camera(&mut self) {
        let cam = CameraNode::new_alloc();
        self.base_mut()
            .add_child(&cam.clone().upcast::<CameraNode>());

        let client = match Engine::singleton().get_singleton("NetworkClient") {
            None => {
                godot_error!("Failed to get singleton");
                return;
            }
            Some(s) => s.try_cast::<NetworkClient>().unwrap(),
        };

        self.network_client = Some(client.clone());
    }

    pub fn reconcile_with_server(&mut self, server_player: Player, delta: f64) {
        self.update_position(Vector3 {
            x: server_player.x,
            y: server_player.y,
            z: server_player.rotation,
        });
        self.update_health(server_player.hp);
        self.data.vx = server_player.vx;
        self.data.vy = server_player.vy;

        let last_ack = server_player.last_processed_input_seq;

        self.pending_inputs.retain(|input| input.seq > last_ack);

        let unack = self.pending_inputs.clone();
        for input in unack.iter() {
            self.apply_local_input(input.action, delta);
        }
    }

    pub fn apply_local_input(&mut self, action: InputAction, delta: f64) {
        match action {
            InputAction::RotateLeft => self.data.rotation -= 0.05,
            InputAction::RotateRight => self.data.rotation += 0.05,
            InputAction::Thrust => {
                let force = 2.0;
                self.data.vx += (force * self.data.rotation.cos()) * delta as f32;
                self.data.vy += (force * self.data.rotation.sin()) * delta as f32;
            }
            InputAction::Shoot => {
                //TODO
            }
            _ => {}
        }

        self.data.update_player_position();
    }
}
