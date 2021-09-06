use macroquad::prelude as mq;

use game_states::{GameState, GameStateEditing, SharedState};
use util::fps_counter::FpsCounter;

use consts::CELL_PX_SIZE;

pub mod consts;
mod game_states;
mod matrix;
pub mod util;

pub struct Game {
    shared_state: SharedState,
    game_state: Box<dyn GameState>,
    fps_counter: FpsCounter,
}

impl Game {
    pub fn new() -> Self {
        Self {
            shared_state: SharedState::new(),
            game_state: Box::new(GameStateEditing::new()),
            fps_counter: FpsCounter::default(),
        }
    }

    pub fn update(&mut self) {
        self.fps_counter.update();

        if let Some(new_game_state) = self.game_state.update(&mut self.shared_state) {
            self.game_state = new_game_state;
        }
    }

    pub fn draw(&self) {
        self.draw_grid();
        self.game_state.draw();
        self.fps_counter.draw();
    }

    fn draw_grid(&self) {
        self.shared_state.matrix.for_each(|x, y, &alive| {
            if !alive {
                return;
            }

            mq::draw_rectangle(
                x as f32 * CELL_PX_SIZE as f32,
                y as f32 * CELL_PX_SIZE as f32,
                CELL_PX_SIZE as f32,
                CELL_PX_SIZE as f32,
                mq::WHITE,
            );
        })
    }
}