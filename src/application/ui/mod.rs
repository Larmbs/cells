mod components;
pub mod style;
mod units;

use components::*;
use style::*;
use units::*;

use macroquad::prelude::*;

const HEADER1_SIZE: f32 = 32.0;
const HEADER2_SIZE: f32 = 24.0;
const HEADER3_SIZE: f32 = 18.0;
const PARAGRAPH_SIZE: f32 = 14.0;
const PADDING: f32 = 8.0;

pub struct UI {
    pub panels: Vec<Panel>,
    style: Style,
}
impl UI {
    pub fn draw(&self) {
        for panel in &self.panels {
            panel.draw(&self.style);
        }
    }
}

pub struct Panel {
    pub hidden: bool,
    pub position: UIUnits,
    pub size: UIUnits,
    pub root_component: Component,
    pub style: Option<Style>,
}
impl Panel {
    fn draw(&self, parents_style: &Style) {
        if self.hidden {
            return;
        }

        let pos = self.position.get_size();
        let size = self.size.get_size();
        // Draw panel background
        draw_rectangle(pos.x, pos.y, size.x, size.y, GRAY);

        // Start drawing components at panel's top-left + padding
        let mut cursor = pos + vec2(PADDING, PADDING);
        let mut container_size = size - (PADDING * 2.0);

        self.root_component.draw(
            &mut cursor,
            container_size,
            if let Some(style) = &self.style {
                style
            } else {
                parents_style
            },
        );
    }
}
