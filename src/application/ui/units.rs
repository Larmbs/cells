use macroquad::prelude::{Vec2, screen_width, screen_height};

pub enum UIUnits {
    Absolute{ x: f32, y: f32},
    /// Should range from 0% (0.00) to 100% (1.00)
    Scaled{ x: f32, y: f32},
}
impl UIUnits {
    pub fn get_size(&self) -> Vec2 {
        match self {
            UIUnits::Absolute{ x, y } => Vec2::new(*x, *y),
            UIUnits::Scaled{ x, y } => Vec2::new(*x * screen_width(), *y * screen_height()),
        }
    }
}
