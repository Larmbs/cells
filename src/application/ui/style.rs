use macroquad::{prelude::Color, text::Font};

#[derive(Clone)]
pub struct TextStyle {
    pub font_size: f32,
    pub font_color: Color,
}

#[derive(Clone)]
pub struct NumberInputStyle {
    pub text_style: TextStyle,
    pub background_color: Color,
    pub border_color: Color,
    pub border_thickness: f32,
    pub padding: f32,
}

#[derive(Clone)]
pub struct TextInputStyle {
    pub text_style: TextStyle,
    pub background_color: Color,
    pub border_color: Color,
    pub border_thickness: f32,
    pub padding: f32,
}

#[derive(Clone)]
pub struct ButtonStyle {
    pub text_style: TextStyle,
    pub background_color: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub border_color: Color,
    pub border_thickness: f32,
    pub padding: f32,
}

#[derive(Clone)]
pub struct Style {
    pub font: Font,
    pub background_color: Color,
    pub border_color: Color,
    pub padding: f32,

    // Typography presets
    pub header1: TextStyle,
    pub header2: TextStyle,
    pub header3: TextStyle,
    pub paragraph: TextStyle,

    // Components
    pub number_input: NumberInputStyle,
    pub text_input: TextInputStyle,
    pub button: ButtonStyle,

    pub number_display: TextStyle,
    pub text_display: TextStyle,
}
