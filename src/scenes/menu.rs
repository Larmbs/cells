use super::Scene;
use crate::{
    AppMessage,
    settings::{Action, KeyBinds},
};
use macroquad::prelude::{BLUE, draw_rectangle};

pub struct Menu {}
impl Scene for Menu {
    fn update(&mut self, key_binds: &KeyBinds) -> AppMessage {
        if key_binds.is_key_pressed(Action::SwitchScene) {
            return AppMessage::OpenEditor(None);
        }
        AppMessage::None
    }

    fn draw(&self) {
        draw_rectangle(0.0, 0.0, 50.0, 50.0, BLUE);
    }
}
impl Menu {
    pub fn new() -> Self {
        Self {}
    }
}
