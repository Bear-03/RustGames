use macroquad::prelude as mq;
use mq::KeyCode;

use common::FpsCounter;

use game_states::{GameState, GameStateIdle, SharedState};
pub use score_manager::ScoreManager;

mod entities;
mod game_states;
mod score_manager;

pub struct Game {
    global: SharedState,
    game_state: Box<dyn GameState>,
    debug_mode: bool,
    fps_counter: FpsCounter,
}

impl Game {
    pub fn new() -> Self {
        Self {
            global: SharedState::new(),
            game_state: Box::new(GameStateIdle::new(true)),
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

        if let Some(new_game_state) = self.game_state.update(&mut self.global) {
            self.game_state = new_game_state;
        }
    }

    pub fn draw(&self) {
        self.global.rackets.0.draw();
        self.global.rackets.1.draw();
        self.global.ball.draw();

        self.global.score_manager.draw();

        self.game_state.draw();

        if self.debug_mode {
            self.draw_debug();
        }
    }

    fn draw_debug(&self) {
        self.fps_counter.draw();

        mq::draw_text(
            &format!("{:?}", self.game_state),
            10.0,
            mq::screen_height() - 10.0,
            30.0,
            mq::GREEN,
        );
    }
}
