use macroquad::prelude as mq;
use mq::{KeyCode, MouseButton};

use super::{GameStateRunning, SharedState};
use crate::{consts::CELL_PX_SIZE, util::Vec2, GameStateBox};
use common::GameState;

/// Returns which cell the mouse is hovering over
fn get_mouse_cell_pos() -> Vec2<usize> {
    let (mouse_x, mouse_y) = mq::mouse_position();

    Vec2::new(
        mouse_x as usize / CELL_PX_SIZE as usize,
        mouse_y as usize / CELL_PX_SIZE as usize,
    )
}

#[derive(Debug)]
pub struct GameStateEditing;

impl GameStateEditing {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_clicks(&self, shared_state: &mut SharedState) {
        if mq::is_mouse_button_down(MouseButton::Left) {
            self.paint(shared_state);
        } else if mq::is_mouse_button_down(MouseButton::Right) {
            self.erase(shared_state);
        }
    }

    fn paint(&self, shared_state: &mut SharedState) {
        let coords = get_mouse_cell_pos();
        shared_state.grid.cells[coords.x][coords.y] = true;
    }

    fn erase(&self, shared_state: &mut SharedState) {
        let coords = get_mouse_cell_pos();
        shared_state.grid.cells[coords.x][coords.y] = false;
    }
}

impl GameState for GameStateEditing {
    type Shared = SharedState;

    fn update(&mut self, shared_state: &mut Self::Shared) -> Option<GameStateBox> {
        self.handle_clicks(shared_state);

        if mq::is_key_pressed(KeyCode::Space) {
            Some(Box::new(GameStateRunning::new()))
        } else {
            None
        }
    }
}
