mod application;

#[macroquad::main("VehicleToolkit")]
async fn main() {
    let mut app = application::Application::new();
    app.run().await;
}
