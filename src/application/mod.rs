#![allow(unused)]
pub mod craft;
mod editor;
mod simulation;

use std::mem;

use macroquad::prelude::{KeyCode, WHITE, clear_background, is_key_pressed};

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
}
impl Application {
    pub fn new() -> Self {
        Self {
            mode: Mode::Edit(editor::Editor::new()),
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
            }
            Mode::Simulate(simulation) => {
                simulation.update();
                simulation.draw();
            }
        }
    }
}
