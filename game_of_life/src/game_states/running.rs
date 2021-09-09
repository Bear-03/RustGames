use macroquad::prelude as mq;
use mq::KeyCode;

use super::{GameStateEditing, SharedState};
use crate::{consts::SECONDS_TO_UPDATE_GRID, grid::Grid, GameStateBox};
use common::{GameState, Timer};

#[derive(Debug)]
pub struct GameStateRunning {
    update_grid_timer: Timer,
}

impl GameStateRunning {
    pub fn new() -> Self {
        Self {
            update_grid_timer: Timer::new(SECONDS_TO_UPDATE_GRID),
        }
    }

    fn update_grid(&self, shared_state: &mut SharedState) {
        let mut new_grid = Grid::new(false);

        shared_state.grid.for_each(|x, y, &alive| {
            let alive_neighbors = shared_state
                .grid
                .count_alive_neighbors(x as isize, y as isize);

            if alive && (2..=3).contains(&alive_neighbors) || !alive && alive_neighbors == 3 {
                new_grid.cells[x][y] = true;
            }
        });

        shared_state.grid = new_grid;
    }
}

impl GameState for GameStateRunning {
    type Shared = SharedState;

    fn update(&mut self, shared_state: &mut Self::Shared) -> Option<GameStateBox> {
        if mq::is_key_pressed(KeyCode::Space) {
            return Some(Box::new(GameStateEditing::new()));
        }

        if self.update_grid_timer.is_over() {
            self.update_grid(shared_state);
        }

        None
    }
}
