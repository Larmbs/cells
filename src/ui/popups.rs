use macroquad::{
    prelude::{screen_height, screen_width, vec2},
    ui::{hash, root_ui},
};

/// Draws an exit process popup to the screen
pub fn exit_popup<T: FnMut(), Y: Fn()>(mut cancel_fn: T, exit_fn: Y) {
    let window_size = vec2(500.0, 370.0);
    let window_pos = vec2(
        screen_width() / 2.0 - window_size.x / 2.0,
        screen_height() / 2.0 - window_size.y / 2.0,
    );

    root_ui().window(hash!("quit_popup"), window_pos, window_size, |ui| {
        ui.label(vec2(20.0, 10.0), "Are you sure you want to quit?");

        if ui.button(vec2(30.0, 90.0), "Cancel") {
            cancel_fn();
        }

        if ui.button(vec2(270.0, 90.0), "Quit") {
            exit_fn();
        }
    });
}
