#![allow(unused)]
pub mod craft;
mod editor;
mod simulation;
pub mod ui;

use macroquad::prelude::next_frame;


pub trait ApplicationMode {
    fn update(&mut self) -> ApplicationMessage;
    fn draw(&self);
}
pub enum ApplicationMessage {
    None,
    Quit,
    MenuMode,
    EditMode,
    SimulateMode,
}

pub struct Application {
    mode: Box<dyn ApplicationMode>,
}
impl Application {
    pub fn new() -> Self {
        Self {
            mode: Box::new(editor::Editor::new()),
        }
    }
    pub async fn run(&mut self) {
        loop {
            match self.mode.update() {
                ApplicationMessage::None => (),
                ApplicationMessage::Quit => break,
                ApplicationMessage::MenuMode => {
                    
                },
                ApplicationMessage::EditMode => {

                },
                ApplicationMessage::SimulateMode => {
                    
                },
            }

            self.mode.draw();
            next_frame().await;
        }
    }
}
