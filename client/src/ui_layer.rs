use godot::{
    classes::{CanvasLayer, ICanvasLayer, Label},
    prelude::*,
};

use crate::player::PlayerWrapper;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct UiLayer {
    base: Base<CanvasLayer>,
    hp_label: Option<Gd<Label>>,
}

#[godot_api]
impl ICanvasLayer for UiLayer {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base,
            hp_label: None,
        }
    }

    fn ready(&mut self) {
        let label = self.base().get_node_as::<Label>("InfoPanel/HpLabel");
        self.hp_label = Some(label);
    }
}

#[godot_api]
impl UiLayer {
    pub fn connect_to_player(&mut self, player: &Gd<PlayerWrapper>) {
        godot_print!("OVER HERE HEALTH");
        player
            .signals()
            .health_updated()
            .connect_other(self, |this, hp| {
                this.on_hp_changed(hp);
            });
    }

    #[func]
    fn on_hp_changed(&mut self, new_hp: u16) {
        godot_print!("health_changed");
        if let Some(label) = &mut self.hp_label {
            label.set_text(&format!("HP: {}", new_hp));
        }
    }
}
