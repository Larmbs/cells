#![allow(unused)]
pub mod craft;
mod editor;
mod simulation;
pub mod ui;
use ui::*;
use ui::style::*;

use std::mem;

use macroquad::{color::Color, prelude::{clear_background, is_key_pressed, KeyCode, Vec2, WHITE}, ui::Style};

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

pub fn application_style() -> Style {
    Style {
        background: Color::from_rgba(77, 75, 76, 128),
        background_hovered: todo!(),
        background_clicked: todo!(),
        color: todo!(),
        color_inactive: todo!(),
        color_hovered: todo!(),
        color_clicked: todo!(),
        color_selected: todo!(),
        color_selected_hovered: todo!(),
        background_margin: todo!(),
        margin: todo!(),
        font: todo!(),
        text_color: todo!(),
        text_color_hovered: todo!(),
        text_color_clicked: todo!(),
        font_size: todo!(),
        reverse_background_z: todo!(),
    }
}