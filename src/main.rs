//! Vehicle Toolkit
//!
//! This Module defines the most basic functionality of an app,
//! it defines this apps basic control structure controlling
//! the swapping between windows / scenes.
//#![allow(unused)]

/* Common modules */
pub mod craft;
pub mod settings;

mod scenes;

pub mod ui;

mod style;
use style::build_application_style;

use craft::Craft;
use macroquad::{
    input::{KeyCode, is_key_pressed},
    prelude::next_frame,
    ui::root_ui,
};

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
    show_quit_popup: bool,
}
impl Application {
    pub async fn init() -> Self {
        let skin = build_application_style().await;
        root_ui().push_skin(&skin);

        Self {
            mode: Box::new(scenes::menu::Menu::new()),
            key_binds: KeyBinds::default(),
            show_quit_popup: false,
        }
    }
    pub async fn run(&mut self) {
        loop {
            // Update with message handling
            if !self.show_quit_popup {
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
            }

            if is_key_pressed(KeyCode::Escape) {
                self.show_quit_popup = !self.show_quit_popup;
            }

            if self.show_quit_popup {
                ui::popups::exit_popup(
                    || {
                        self.show_quit_popup = false;
                    },
                    || {
                        std::process::exit(0);
                    },
                );
            }

            // Drawing to screen
            self.mode.draw();
            next_frame().await;
        }
    }
}

#[macroquad::main("VehicleToolkit")]
async fn main() {
    let mut app = Application::init().await;
    app.run().await;
}
