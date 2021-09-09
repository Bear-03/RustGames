use macroquad::prelude as mq;

use crate::{
    entities::{Ball, Collider, RacketPair},
    GameStateBox, ScoreManager,
};

use super::{GameStateEnded, GameStateIdle, SharedState};
use common::GameState;

#[derive(Debug)]
pub struct GameStateRunning {
    ball_has_collided: bool,
}

impl GameStateRunning {
    pub fn new() -> Self {
        Self {
            ball_has_collided: false,
        }
    }

    #[allow(unused_must_use)]
    fn check_ball_collisions(&mut self, shared_state: &mut SharedState) {
        if self.check_ball_side_collision(&mut shared_state.ball) {
            return;
        };
        self.check_ball_racket_collision(&shared_state.rackets, &mut shared_state.ball);
    }

    fn check_ball_side_collision(&mut self, ball: &mut Ball) -> bool {
        if ball.pos.y <= 0.0 || ball.pos.y >= mq::screen_height() - ball.size {
            ball.direction.y = -ball.direction.y;
            return true;
        }
        false
    }

    fn check_ball_racket_collision(&mut self, rackets: &RacketPair, ball: &mut Ball) -> bool {
        let collides = rackets.0.collides_with(ball) || rackets.1.collides_with(ball);

        if collides && !self.ball_has_collided {
            ball.direction.x = -ball.direction.x;
            self.ball_has_collided = true;
        } else if !collides && self.ball_has_collided {
            self.ball_has_collided = false;
        }

        collides
    }

    fn update_score(
        &mut self,
        ball: &mut Ball,
        score_manager: &mut ScoreManager,
    ) -> Option<(u32, u32)> {
        if ball.pos.x < 0.0 {
            Some((2, score_manager.player2.increase_score()))
        } else if ball.pos.x > mq::screen_width() {
            Some((1, score_manager.player1.increase_score()))
        } else {
            None
        }
    }
}

impl GameState for GameStateRunning {
    type Shared = SharedState;

    fn update(&mut self, shared_state: &mut Self::Shared) -> Option<GameStateBox> {
        shared_state.ball.update();

        shared_state.rackets.0.update();
        shared_state.rackets.1.update();

        self.check_ball_collisions(shared_state);

        match self.update_score(&mut shared_state.ball, &mut shared_state.score_manager) {
            Some((player_num, score)) => {
                shared_state.ball = Ball::new();
                if score == 5 {
                    Some(Box::new(GameStateEnded::new(player_num)))
                } else {
                    Some(Box::new(GameStateIdle::new(false)))
                }
            }
            None => None,
        }
    }
}
