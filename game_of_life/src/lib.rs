use macroquad::prelude as mq;
use mq::KeyCode;

use game_states::{GameState, GameStateEditing, SharedState};
use util::fps_counter::FpsCounter;

use consts::CELL_PX_SIZE;

pub mod consts;
mod game_states;
mod grid;
pub mod util;

pub struct Game {
    shared_state: SharedState,
    game_state: Box<dyn GameState>,
    debug_mode: bool,
    fps_counter: FpsCounter,
}

impl Game {
    pub fn new() -> Self {
        Self {
            shared_state: SharedState::new(),
            game_state: Box::new(GameStateEditing::new()),
            debug_mode: false,
            fps_counter: FpsCounter::default(),
        }
    }

    pub fn update(&mut self) {
        if mq::is_key_pressed(KeyCode::F1) {
            self.debug_mode = !self.debug_mode;
        }

        if self.debug_mode {
            self.fps_counter.update();
        }

        if let Some(new_game_state) = self.game_state.update(&mut self.shared_state) {
            self.game_state = new_game_state;
        }
    }

    pub fn draw(&self) {
        self.draw_grid();
        self.game_state.draw();

        if self.debug_mode {
            self.fps_counter.draw();
        }
    }

    fn draw_grid(&self) {
        self.shared_state.grid.for_each(|x, y, &alive| {
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
