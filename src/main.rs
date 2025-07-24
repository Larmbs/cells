use macroquad::prelude::*;
mod application;

#[macroquad::main("Advanced Physics Builder")]
async fn main() {
    let mut app = application::Application::new();

    loop {
        app.update();
        next_frame().await;
    }
}
