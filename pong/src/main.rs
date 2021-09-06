use macroquad::prelude as mq;
use mq::Conf;

use pong::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong!".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();

        mq::clear_background(mq::BLACK);
        game.draw();

        mq::next_frame().await
    }
}
