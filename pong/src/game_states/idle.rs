use macroquad::prelude as mq;
use mq::vec2;

use super::{GameState, GameStateRunning, SharedState};
use crate::util::Text;

pub const TEXT_Y_OFFSET: f32 = 100.0;
pub const FONT_SIZE: u16 = 100;

#[derive(Debug)]
pub struct GameStateIdle {
    text: Option<Text>,
}

impl GameStateIdle {
    pub fn new(in_menu: bool) -> Self {
        Self {
            text: if in_menu {
                Some(Text::new(
                    "Pong!",
                    vec2(
                        mq::screen_width() / 2.0,
                        mq::screen_height() / 2.0 - TEXT_Y_OFFSET,
                    ),
                    FONT_SIZE,
                    mq::WHITE,
                ))
            } else {
                None
            },
        }
    }
}

impl GameState for GameStateIdle {
    fn update(&mut self, shared_state: &mut SharedState) -> Option<Box<dyn GameState>> {
        let movement_leys_pressed = shared_state
            .rackets
            .get_all_controls()
            .iter()
            .any(|&key| mq::is_key_down(key));

        if movement_leys_pressed {
            Some(Box::new(GameStateRunning::new()))
        } else {
            None
        }
    }

    fn draw(&self) {
        if let Some(text) = &self.text {
            text.draw();
        }
    }
}
