use macroquad::prelude::{BLUE, KeyCode, draw_rectangle, is_key_pressed};

use crate::application::{menu, AppMessage, Scene};

pub struct Menu {}
impl Scene for Menu {
    fn update(&mut self) -> AppMessage {
        if is_key_pressed(KeyCode::Space) {
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
        Self{}
    }
}
