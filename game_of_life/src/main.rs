use macroquad::prelude as mq;
use mq::Conf;

use game_of_life::{
    consts::{CELL_PX_SIZE, GRID_SIZE},
    Game,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: CELL_PX_SIZE as i32 * GRID_SIZE.x as i32,
        window_height: CELL_PX_SIZE as i32 * GRID_SIZE.y as i32,
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
