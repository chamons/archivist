use macroquad::{miniquad::conf::Icon, window::Conf};

fn window_conf() -> Conf {
    Conf {
        window_title: "The Archivist".to_string(),
        window_width: 1024,
        window_height: 800,
        icon: Some(Icon {
            big: include_bytes!("../icon/64").clone(),
            medium: include_bytes!("../icon/32").clone(),
            small: include_bytes!("../icon/16").clone(),
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    archivist::prelude::main().await;
}
