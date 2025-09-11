use macroquad::window::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "The Archivist".to_string(),
        window_width: 1024,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    archivist::prelude::main().await;
}
