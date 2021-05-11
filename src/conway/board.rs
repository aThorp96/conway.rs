use rand::prelude::*;
use std::fmt;

use crate::conway::conway::{Cell, CellState, Coordinate, SimpleCell};

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<SimpleCell>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Vec<SimpleCell>> = Vec::with_capacity(height);

        // Initialize vectors
        for i in 0..height {
            let mut row = Vec::with_capacity(width);
            for j in 0..width {
                row.push(SimpleCell::new(Coordinate::new(i as i32, j as i32)));
            }
            cells.push(row);
        }

        Board {
            width,
            height,
            cells: cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<SimpleCell> {
        if x < self.width && x >= 0 && y < self.height && y >= 0 {
            Some(self.cells[x][y].clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, state: CellState) {
        self.cells[x][y].set_state(state);
    }

    pub fn step(&mut self) {
        let mut cells = self.cells.clone();
        let mut display = String::new();
        for i in 0..cells.len() {
            for j in 0..cells[i].len() {
                let cell = cells[i][j].clone();
                cells[i][j].set_state(self.next_cell_step(cell));
            }
        }
        self.cells = cells;
    }

    fn next_cell_step(&self, cell: SimpleCell) -> CellState {
        let adj = cell.get_adjecent();
        let neighbors = self.get_cells(adj);
        let live_neighbors = neighbors.iter().filter(|c| c.is_alive()).count();

        // birth
        if !cell.is_alive() && live_neighbors == 3 {
            CellState::Alive
            // Starvation
        } else if cell.is_alive() && live_neighbors > 3 {
            CellState::Dead
            // Isolation
        } else if cell.is_alive() && live_neighbors <= 1 {
            CellState::Dead
            // Survival
        } else {
            cell.state
        }
    }

    fn get_cells(&self, coordinates: Vec<Coordinate>) -> Vec<SimpleCell> {
        let mut cells = Vec::new();
        for c in coordinates {
            if let Some(cell) = self.get(c.x as usize, c.y as usize) {
                cells.push(cell);
            }
        }
        cells
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cells = self.cells.clone();
        let mut display = String::new();
        for row in cells {
            for cell in row {
                if cell.is_alive() {
                    display.push_str("()");
                } else {
                    display.push_str("__");
                }
            }
            display.push('\n');
        }
        write!(f, "{}", display)
    }
}
