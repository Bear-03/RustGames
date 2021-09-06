use crate::{consts::RANDOMIZE_GRID, matrix::Matrix};

pub struct SharedState {
    pub matrix: Matrix,
}

impl SharedState {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            matrix: Matrix::new(RANDOMIZE_GRID),
        }
    }
}
