use macroquad::prelude as mq;
use mq::{KeyCode, MouseButton};

use super::{GameState, GameStateRunning, SharedState};
use crate::{consts::CELL_PX_SIZE, util::Vec2};

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

    fn handle_clicks(&self, global: &mut SharedState) {
        if mq::is_mouse_button_down(MouseButton::Left) {
            self.paint(global);
        } else if mq::is_mouse_button_down(MouseButton::Right) {
            self.erase(global);
        }
    }

    fn paint(&self, global: &mut SharedState) {
        let coords = get_mouse_cell_pos();
        global.matrix.cells[coords.x][coords.y] = true;
    }

    fn erase(&self, global: &mut SharedState) {
        let coords = get_mouse_cell_pos();
        global.matrix.cells[coords.x][coords.y] = false;
    }
}

impl GameState for GameStateEditing {
    fn update(&mut self, global: &mut SharedState) -> Option<Box<dyn GameState>> {
        self.handle_clicks(global);

        if mq::is_key_pressed(KeyCode::Space) {
            Some(Box::new(GameStateRunning::new()))
        } else {
            None
        }
    }
}
