use macroquad::prelude::{Rect, Vec2};

pub trait UiWidget {
    fn update(&mut self, mouse: Vec2, mouse_pressed: bool);
    fn draw(&self);
}

struct Text(String);
impl Text {
    fn new();
}
impl UiWidget for Text {
    fn update(&mut self, mouse: Vec2, mouse_pressed: bool) {
        todo!()
    }

    fn draw(&self) {
        todo!()
    }
}

struct DisplayText {
    pub rect: Rect,
    pub get_text: Box<dyn Fn() -> String>,
}

struct Button {
    pub label: String,
    pub rect: Rect,
    pub on_click: Option<Box<dyn FnMut()>>,
    pub hovered: bool,
}
impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.into(),
            rect: Rect::new(0.0, 0.0, 100.0, 40.0),
            on_click: None,
            hovered: false,
        }
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}
impl UiWidget for Button {
    fn update(&mut self, mouse: Vec2, mouse_pressed: bool) {
        self.hovered = self.rect.contains(mouse);
        if self.hovered && mouse_pressed {
            if let Some(callback) = &mut self.on_click {
                callback();
            }
        }
    }

    fn draw(&self) {}
}
