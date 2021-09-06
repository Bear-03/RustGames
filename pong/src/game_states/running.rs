use macroquad::prelude as mq;

use crate::entities::{Ball, Collider, RacketPair};
use crate::Global;
use crate::ScoreManager;

use super::{GameState, GameStateEnded, GameStateIdle};

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
    fn check_ball_collisions(&mut self, global: &mut Global) {
        if self.check_ball_side_collision(&mut global.ball) {
            return;
        };
        self.check_ball_racket_collision(&global.rackets, &mut global.ball);
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
    fn update(&mut self, global: &mut Global) -> Option<Box<dyn GameState>> {
        global.ball.update();

        global.rackets.0.update();
        global.rackets.1.update();

        self.check_ball_collisions(global);

        match self.update_score(&mut global.ball, &mut global.score_manager) {
            Some((player_num, score)) => {
                global.ball = Ball::new();
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
