use flipper::Flipper;
use flipperspiel::*;

use macroquad::window;

mod flipper;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Flipperspiel".to_owned(),
        high_dpi: true,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    info!("Launched!");
    let flipper = Flipper::new();
    loop {
        info!("Inside loop!");
        clear_background(WHITE);
        flipper.draw();
        next_frame().await;
    }
}
