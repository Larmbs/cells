use macroquad::{
    prelude::{RectOffset, WHITE, load_file, load_image},
    ui::{Skin, root_ui},
};

/// Function that defines UI styling
pub async fn build_application_style() -> Skin {
    let window_background = load_image("./assets/ui/window_bg.png").await.unwrap();
    let button_background = load_image("./assets/ui/button_bg.png").await.unwrap();
    let button_clicked_background = load_image("./assets/ui/button_clicked_bg.png")
        .await
        .unwrap();
    let button_hovered_background = load_image("./assets/ui/button_hovered_bg.png")
        .await
        .unwrap();

    let font = load_file("assets/ui/fonts/ShareTech-Regular.ttf")
        .await
        .unwrap();

    let window_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
        .background(window_background)
        .font(&font)
        .unwrap()
        .build();
    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .background(button_background)
        .background_clicked(button_clicked_background)
        .background_hovered(button_hovered_background)
        .text_color(WHITE)
        .font(&font)
        .unwrap()
        .font_size(32)
        .build();
    let label_style = root_ui()
        .style_builder()
        .text_color(WHITE)
        .font(&font)
        .unwrap()
        .font_size(32)
        .build();
    Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    }
}
