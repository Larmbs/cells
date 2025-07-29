//!
//!
pub mod editor;
pub mod menu;
pub mod simulation;

use crate::{AppMessage, settings::KeyBinds};

/// Definition of a basic window for the program,
/// The application manages switching between these scenes.
pub trait Scene {
    fn update(&mut self, key_binds: &KeyBinds) -> AppMessage;
    fn draw(&self);
}
