mod application;

#[macroquad::main("Advanced Physics Builder")]
async fn main() {
    let mut app = application::Application::new();
    app.run().await;
}
