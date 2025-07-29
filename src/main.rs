//! Vehicle Toolkit
//!
//! This Module defines the most basic functionality of an app,
//! it defines this apps basic control structure controlling
//! the swapping between windows / scenes.
//#![allow(unused)]

/* Common modules */
pub mod craft;
pub mod settings;
pub mod ui;

mod scenes;

use craft::Craft;
use macroquad::prelude::next_frame;
use settings::KeyBinds;

/// These are messages that are passed from the scene to the application
/// To execute some sort of action that it has no ability too at its level
pub enum AppMessage {
    None,
    Quit,
    OpenMenu,
    OpenEditor(Option<Craft>),
    OpenSimulation(Craft),
}

pub struct Application {
    mode: Box<dyn scenes::Scene>,
    key_binds: KeyBinds,
}
impl Application {
    pub fn new() -> Self {
        Self {
            mode: Box::new(scenes::menu::Menu::new()),
            key_binds: KeyBinds::default(),
        }
    }
    pub async fn run(&mut self) {
        loop {
            // Update with message handling
            match self.mode.update(&self.key_binds) {
                AppMessage::None => (),
                AppMessage::Quit => break,
                AppMessage::OpenMenu => self.mode = Box::new(scenes::menu::Menu::new()),
                AppMessage::OpenEditor(craft) => {
                    if let Some(craft) = craft {
                        self.mode = Box::new(scenes::editor::Editor::edit_craft(craft))
                    } else {
                        self.mode = Box::new(scenes::editor::Editor::new())
                    }
                }
                AppMessage::OpenSimulation(craft) => {
                    self.mode = Box::new(scenes::simulation::Simulation::new(craft))
                }
            }

            // Drawing to screen
            self.mode.draw();
            next_frame().await;
        }
    }
}

#[macroquad::main("VehicleToolkit")]
async fn main() {
    let mut app = Application::new();
    app.run().await;
}
