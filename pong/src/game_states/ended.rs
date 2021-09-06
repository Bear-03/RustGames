use macroquad::prelude as mq;
use mq::vec2;

use super::{GameState, GameStateIdle, SharedState};
use crate::util::Text;

pub const TEXT_Y_OFFSET: f32 = 100.0;
pub const FONT_SIZE: u16 = 70;

#[derive(Debug)]
pub struct GameStateEnded {
    text: Text,
    seconds_until_reset: f32,
}

impl GameStateEnded {
    pub fn new(player_num: u32) -> Self {
        Self {
            text: Text::new(
                &format!("Player {} wins!", player_num),
                vec2(
                    mq::screen_width() / 2.0,
                    mq::screen_height() / 2.0 - TEXT_Y_OFFSET,
                ),
                FONT_SIZE,
                mq::WHITE,
            ),
            seconds_until_reset: 5.0,
        }
    }
}

impl GameState for GameStateEnded {
    fn update(&mut self, shared_state: &mut SharedState) -> Option<Box<dyn GameState>> {
        self.seconds_until_reset -= mq::get_frame_time();

        if self.seconds_until_reset < 0.0 {
            *shared_state = SharedState::new();
            Some(Box::new(GameStateIdle::new(true)))
        } else {
            None
        }
    }

    fn draw(&self) {
        self.text.draw();
    }
}
