use macroquad::ui::{InputHandler, root_ui};

pub mod units;
use units::UiUnits;

pub struct Ui {
    ui_components: Vec<UiComponent>,
}
impl Ui {
    
}

struct UiComponent {
    position: UiUnits,
    dimensions: UiUnits,
    root_element: Element,
}

enum Element {
    /* Static */
    Text { label: String },

    /* Display */
    NumberDisplay { current: f32 },
    TextDisplay { current: String },

    /* Inputs */
    NumberInput { label: String, min: f32, max: f32 },
    NumberSlider { label: String, min: f32, max: f32 },
    TextInput { label: String },
    SelectBox { label: String, options: Vec<String> },
    Button { label: String },

    /* Format */
    Row { elements: Vec<Element> },
    Column { elements: Vec<Element> },
}
impl Element {
    fn draw(&self) {
        match self {
            Element::Text { label } => todo!(),

            Element::NumberDisplay { current } => todo!(),
            Element::TextDisplay { current } => todo!(),

            Element::NumberInput { label, min, max } => todo!(),
            Element::NumberSlider { label, min, max } => todo!(),
            Element::TextInput { label } => todo!(),
            Element::SelectBox { label, options } => todo!(),
            Element::Button { label } => todo!(),

            Element::Row { elements } => todo!(),
            Element::Column { elements } => todo!(),
        }
    }
}
