#![allow(unused)]
pub mod craft;
mod editor;
mod simulation;
pub mod ui;
use ui::*;

use std::mem;

use macroquad::prelude::{KeyCode, WHITE, clear_background, is_key_pressed, Vec2};

enum Mode {
    Edit(editor::Editor),
    Simulate(simulation::Simulation),
}
impl Mode {
    pub fn swap(self) -> Self {
        match self {
            Mode::Edit(editor) => Mode::Simulate(simulation::Simulation::new(editor.close())),
            Mode::Simulate(simulation) => {
                Mode::Edit(editor::Editor::edit_craft(simulation.close()))
            }
        }
    }
}

pub struct Application {
    mode: Mode,
    ui: UI,
}
impl Application {
    pub fn new() -> Self {
        Self {
            mode: Mode::Edit(editor::Editor::new()),
            ui: Self::default_application_ui(),
        }
    }
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            let mode = std::mem::replace(&mut self.mode, Mode::Edit(editor::Editor::new()));
            self.mode = mode.swap();
        }

        match &mut self.mode {
            Mode::Edit(editor) => {
                editor.update();
                editor.draw();
                self.ui.draw();
            }
            Mode::Simulate(simulation) => {
                simulation.update();
                simulation.draw();
                self.ui.draw();
            }
        }
    }

    fn default_application_ui() -> UI {
        UI {
            panels: vec![Panel {
                hidden: false,
                position: UIUnits::Scaled { x: 0.5 - 0.1, y: 0.5 - 0.05 },
                size: UIUnits::Scaled { x: 0.2, y: 0.1 },
                root_component: Component::Row {
                    components: vec![
                        Component::Button {
                            id: 0,
                            text: String::from("exit"),
                        },
                        Component::Button {
                            id: 1,
                            text: String::from("cancel"),
                        },
                    ],
                },
            }],
        }
    }
}
