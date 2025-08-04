


### Static
Text {
    text: String,
}
Break;

### Displays
DisplayText {
    label: String,
    get_text: Fn,
}
DisplayNumber {
    label: String,
    get_number: Fn,
}
ProgressBar {
    label: String,
    last_value: f32,
    get_value: f32,
}

### Inputs
Button {
    label: String,
    on_press: Fn,
}
TextInput {
    label: String,
    max_len: usize,
    value: String,
    on_submit: Fn,
}
NumberInput {
    label: String,
    max: f32,
    min: f32,
    value: f32,
    on_submit: Fn,
}
Slider {
    label: String,
    max: f32,
    min: f32,
    value: f32,
    on_change: Fn,
}
Toggle {
    label: String,
    state: bool,
    on_toggle: Fn,
}
