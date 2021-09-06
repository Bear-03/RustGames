use crate::entities::{Ball, RacketPair};
use crate::ScoreManager;

pub struct SharedState {
    pub rackets: RacketPair,
    pub ball: Ball,
    pub score_manager: ScoreManager,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            rackets: RacketPair::new(),
            ball: Ball::new(),
            score_manager: ScoreManager::new(),
        }
    }
}
