use crate::consts::GRID_SIZE;

type Cells = [[bool; GRID_SIZE.y]; GRID_SIZE.x];

fn wrap_around<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        max
    } else if value > max {
        min
    } else {
        value
    }
}

pub struct Matrix {
    pub cells: Cells,
}

impl Matrix {
    pub fn new(randomize: bool) -> Self {
        let mut matrix = Self {
            cells: [[false; GRID_SIZE.x]; GRID_SIZE.y],
        };

        if randomize {
            matrix.randomize();
        }

        matrix
    }

    fn randomize(&mut self) {
        self.for_each_mut(|_, _, alive| {
            *alive = rand::random();
        })
    }

    pub fn for_each<F: FnMut(usize, usize, &bool)>(&self, mut closure: F) {
        for (x, vec) in self.cells.iter().enumerate() {
            for (y, alive) in vec.iter().enumerate() {
                closure(x, y, alive);
            }
        }
    }

    pub fn for_each_mut<F: FnMut(usize, usize, &mut bool)>(&mut self, mut closure: F) {
        for (x, vec) in self.cells.iter_mut().enumerate() {
            for (y, alive) in vec.iter_mut().enumerate() {
                closure(x, y, alive);
            }
        }
    }

    pub fn get_wrap(&self, x: isize, y: isize) -> bool {
        let x = wrap_around(x, 0, (self.cells.len() - 1) as isize) as usize;
        let y = wrap_around(y, 0, (self.cells[x].len() - 1) as isize) as usize;

        self.cells[x][y]
    }

    pub fn count_alive_neighbors(&self, x: isize, y: isize) -> usize {
        let neighbor_values = [
            self.get_wrap(x, y - 1),
            self.get_wrap(x + 1, y - 1),
            self.get_wrap(x + 1, y),
            self.get_wrap(x + 1, y + 1),
            self.get_wrap(x, y + 1),
            self.get_wrap(x - 1, y + 1),
            self.get_wrap(x - 1, y),
            self.get_wrap(x - 1, y - 1),
        ];

        neighbor_values.iter().filter(|&&e| e).count()
    }
}
