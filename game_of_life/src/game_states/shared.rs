use crate::{consts::RANDOMIZE_GRID, grid::Grid};

pub struct SharedState {
    pub grid: Grid,
}

impl SharedState {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            grid: Grid::new(RANDOMIZE_GRID),
        }
    }
}
