use godot::engine::{Button, CanvasLayer, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    #[base]
    base: Base<CanvasLayer>,
}

#[godot_api]
impl Hud {
    #[signal]
    fn start_game();

    #[func]
    pub fn show_message(&self, text: GodotString) {
        let mut message_label = self.base.get_node_as::<Label>("MessageLabel");
        message_label.set_text(text);
        message_label.show();

        let mut timer = self.base.get_node_as::<Timer>("MessageTimer");
        timer.start(0.0);
    }

    pub fn show_game_over(&self) {
        self.show_message("Game Over".into());

        let mut message_label = self.base.get_node_as::<Label>("MessageLabel");
        message_label.set_text("Dodge the\nCreeps!".into());
        message_label.show();

        let mut button = self.base.get_node_as::<Button>("StartButton");
        button.show();
    }

    #[func]
    pub fn update_score(&self, score: i64) {
        let mut label = self.base.get_node_as::<Label>("ScoreLabel");

        label.set_text(score.to_string().into());
    }

    #[func]
    fn on_start_button_p