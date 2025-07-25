#![allow(unused)]

use crate::application::craft::Craft;
pub mod craft;
pub mod ui;

/* Scenes */
mod menu;
mod editor;
mod simulation;

pub trait Scene {
    fn update(&mut self) -> AppMessage;
    fn draw(&self);
}
pub enum AppMessage {
    None,
    Quit,
    OpenMenu,
    OpenEditor(Option<Craft>),
    OpenSimulation(Craft)
}

pub struct Application {
    mode: Box<dyn Scene>,
}
impl Application {
    pub fn new() -> Self {
        Self {
            mode: Box::new(menu::Menu::new()),
        }
    }
    pub async fn run(&mut self) {
        loop {
            match self.mode.update() {
                AppMessage::None => (),
                AppMessage::Quit => break,
                AppMessage::OpenMenu => {
                    self.mode = Box::new(menu::Menu::new())
                },
                AppMessage::OpenEditor(craft) => {
                    if let Some(craft) = craft {
                        self.mode = Box::new(editor::Editor::edit_craft(craft))
                    } else {
                        self.mode = Box::new(editor::Editor::new())
                    }
                },
                AppMessage::OpenSimulation(craft) => {
                    self.mode = Box::new(simulation::Simulation::new(craft))
                },
            }

            self.mode.draw();
            macroquad::prelude::next_frame().await;
        }
    }
}
