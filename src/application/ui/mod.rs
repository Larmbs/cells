use macroquad::prelude::*;

const HEADER1_SIZE: f32 = 32.0;
const HEADER2_SIZE: f32 = 24.0;
const HEADER3_SIZE: f32 = 18.0;
const PARAGRAPH_SIZE: f32 = 14.0;
const PADDING: f32 = 8.0;

pub enum UIUnits {
    Absolute{ x: f32, y: f32},
    /// Should range from 0% (0.00) to 100% (1.00)
    Scaled{ x: f32, y: f32},
}
impl UIUnits {
    fn get_size(&self) -> Vec2 {
        match self {
            UIUnits::Absolute{ x, y } => Vec2::new(*x, *y),
            UIUnits::Scaled{ x, y } => Vec2::new(*x * screen_width(), *y * screen_height()),
        }
    }
}
pub struct UI {
    pub panels: Vec<Panel>,
}
impl UI {
    pub fn draw(&self) {
        for panel in &self.panels {
            panel.draw();
        }
    }
}

pub struct Panel {
    pub hidden: bool,
    pub position: UIUnits,
    pub size: UIUnits,
    pub root_component: Component,
}
impl Panel {
    fn draw(&self) {
        if self.hidden {
            return;
        }

        let pos = self.position.get_size();
        let size = self.size.get_size();
        // Draw panel background
        draw_rectangle(
            pos.x,
            pos.y,
            size.x,
            size.y,
            GRAY,
        );

        // Start drawing components at panel's top-left + padding
        let mut cursor = pos + vec2(PADDING, PADDING);
        self.root_component.draw(&mut cursor);
    }
}


pub enum Component {
    Row {
        components: Vec<Component>,
    },
    Column {
        components: Vec<Component>,
    },
    Expandable {
        expanded: bool,
        components: Vec<Component>,
    },
    Header1 {
        text: String,
    },
    Header2 {
        text: String,
    },
    Header3 {
        text: String,
    },
    Paragraph {
        text: String,
    },
    NumberInput {
        id: usize,
        current: f32,
        min: f32,
        max: f32,
    },
    TextInput {
        id: usize,
        current: String,
    },
    Button {
        id: usize,
        text: String,
    },
}
impl Component {
    fn draw(&self, cursor: &mut Vec2) {
        match self {
            Component::Row { components } => {
                let mut row_cursor = *cursor;
                let mut max_height: f32 = 0.0;
                for component in components {
                    let mut component_cursor = row_cursor;
                    component.draw(&mut component_cursor);

                    // Move cursor to the right
                    row_cursor.x = component_cursor.x + PADDING;
                    max_height = max_height.max(component_cursor.y - cursor.y);
                }
                // Move main cursor down by the height of the tallest row item
                cursor.y += max_height + PADDING;
            }

            Component::Column { components } => {
                for component in components {
                    component.draw(cursor);
                    cursor.y += PADDING;
                }
            }

            Component::Expandable {
                expanded,
                components,
            } => {
                if *expanded {
                    for component in components {
                        component.draw(cursor);
                        cursor.y += PADDING;
                    }
                }
            }

            Component::Header1 { text } => {
                draw_text(
                    text,
                    cursor.x,
                    cursor.y + HEADER1_SIZE,
                    HEADER1_SIZE,
                    BLACK,
                );
                cursor.y += HEADER1_SIZE + PADDING;
            }

            Component::Header2 { text } => {
                draw_text(
                    text,
                    cursor.x,
                    cursor.y + HEADER2_SIZE,
                    HEADER2_SIZE,
                    BLACK,
                );
                cursor.y += HEADER2_SIZE + PADDING;
            }

            Component::Header3 { text } => {
                draw_text(
                    text,
                    cursor.x,
                    cursor.y + HEADER3_SIZE,
                    HEADER3_SIZE,
                    BLACK,
                );
                cursor.y += HEADER3_SIZE + PADDING;
            }

            Component::Paragraph { text } => {
                draw_text(
                    text,
                    cursor.x,
                    cursor.y + PARAGRAPH_SIZE,
                    PARAGRAPH_SIZE,
                    DARKGRAY,
                );
                cursor.y += PARAGRAPH_SIZE + PADDING;
            }

            Component::NumberInput { id, current, min, max } => {
                let display = format!("{}: {:.2} ({:.1}-{:.1})", id, current, min, max);
                draw_rectangle(cursor.x, cursor.y, 200.0, 30.0, WHITE);
                draw_text(&display, cursor.x + 4.0, cursor.y + 22.0, 16.0, BLACK);
                cursor.y += 30.0 + PADDING;
            }

            Component::TextInput { id, current } => {
                let display = format!("{}: {}", id, current);
                draw_rectangle(cursor.x, cursor.y, 200.0, 30.0, LIGHTGRAY);
                draw_text(&display, cursor.x + 4.0, cursor.y + 22.0, 16.0, BLACK);
                cursor.y += 30.0 + PADDING;
            }

            Component::Button { id, text } => {
                draw_rectangle(cursor.x, cursor.y, 120.0, 32.0, BLUE);
                draw_text(text, cursor.x + 8.0, cursor.y + 22.0, 18.0, WHITE);
                cursor.y += 32.0 + PADDING;
            }
        }
    }
}
