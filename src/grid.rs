use crate::cell::{Cell, CellState};

#[derive(Clone, Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(seed: &[Vec<Cell>]) -> Self {
        Self {
            cells: seed.to_owned(),
        }
    }

    pub fn rows_size(&self) -> usize {
        self.cells[0].len()
    }

    pub fn cols_size(&self) -> usize {
        self.cells.len()
    }

    pub fn randomize(&mut self) {
        self.cells = (0..(self.cells.len()))
            .map(|_| {
                (0..self.cells[0].len())
                    .map(|_| Cell::new_with_random_state())
                    .collect()
            })
            .collect();
    }

    pub fn new_with_random_seed(width: usize, height: usize) -> Self {
        Self {
            cells: (0..(width))
                .map(|_| (0..height).map(|_| Cell::new_with_random_state()).collect())
                .collect(),
        }
    }

    fn upper_left(&self, row_index: usize, col_index: usize) -> Option<&Cell> {
        if row_index == 0 || col_index == 0 {
            return None;
        }

        self.cells.get(row_index - 1)?.get(col_index - 1)
    }

    fn upper(&self, row_index: usize, col_index: usize) -> Option<&Cell> {
        if row_index == 0 {
            return None;
        }

        self.cells.get(row_index - 1)?.get(col_index)
    }

    fn upper_right(&self, row_index: usize, col_index: usize) -> Option<&Cell> {
        if row_index == 0 {
            return None;
        }

        self.cells.get(row_index - 1)?.get(col_index + 1)
    }

    fn left(&self, row_index: usize, col_index: usize) -> Option<&Cell> {
        if col_index == 0 {
            return None;
        }
        self.cells.get(row_index)?.get(col_index - 1)
    }

    fn right(&self, row_index: usize, col_index: usize) -> Option<&Cell> {
        self.cells.get(row_index)?.get(col_index + 1)
    }

    fn lower_left(&self, row_index: usize, col_index: usize) -> Option<&Cell> {
        if col_index == 0 {
            return None;
        }

        self.cells.get(row_index + 1)?.get(col_index - 1)
    }

    fn lower(&self, row_index: usize, col_index: usize) -> Option<&Cell> {
        self.cells.get(row_index + 1)?.get(col_index)
    }

    fn lower_right(&self, row_index: usize, col_index: usize) -> Option<&Cell> {
        self.cells.get(row_index + 1)?.get(col_index + 1)
    }

    fn count_neighbors(&self, row_index: usize, col_index: usize) -> u32 {
        let mut neighbors_count = 0;

        // TODO: here i should be calling the cell's state so it mutates

        if let Some(cell) = self.upper_left(row_index, col_index) {
            if cell.state == CellState::Alive {
                neighbors_count += 1;
            }
        }

        if let Some(cell) = self.upper(row_index, col_index) {
            if cell.state == CellState::Alive {
                neighbors_count += 1;
            }
        }

        if let Some(cell) = self.upper_right(row_index, col_index) {
            if cell.state == CellState::Alive {
                neighbors_count += 1;
            }
        }

        if let Some(cell) = self.left(row_index, col_index) {
            if cell.state == CellState::Alive {
                neighbors_count += 1;
            }
        }

        if let Some(cell) = self.right(row_index, col_index) {
            if cell.state == CellState::Alive {
                neighbors_count += 1;
            }
        }

        if let Some(cell) = self.lower_left(row_index, col_index) {
            if cell.state == CellState::Alive {
                neighbors_count += 1;
            }
        }

        if let Some(cell) = self.lower(row_index, col_index) {
            if cell.state == CellState::Alive {
                neighbors_count += 1;
            }
        }

        if let Some(cell) = self.lower_right(row_index, col_index) {
            if cell.state == CellState::Alive {
                neighbors_count += 1;
            }
        }

        neighbors_count
    }

    pub fn update(&mut self) {
        let mut new_cells = self.cells.clone();

        for row in 0..self.cells.len() {
            for col in 0..self.cells[0].len() {
                let current_cell = &mut new_cells[row][col];
                current_cell.state = match (current_cell.state, self.count_neighbors(row, col)) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (CellState::Alive, x) if x < 2 => CellState::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    // (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (CellState::Alive, x) if x > 3 => CellState::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (CellState::Dead, 3) => CellState::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                }
            }
        }

        self.cells = new_cells;
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    use super::*;

    #[test]
    fn test_new_grid_with_a_seed_should_produce_correct_grid() {
        //tests sizes
    }

    #[test]
    fn test_count_neighbors() {
        let seed = vec![
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
            ],
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
        ];

        let grid = Grid::new(&seed);

        assert_eq!(grid.count_neighbors(0, 0), 1);
        assert_eq!(grid.count_neighbors(0, 1), 3);
        assert_eq!(grid.count_neighbors(0, 2), 2);
        assert_eq!(grid.count_neighbors(1, 0), 4);
        assert_eq!(grid.count_neighbors(1, 1), 4);
        assert_eq!(grid.count_neighbors(1, 2), 2);
        assert_eq!(grid.count_neighbors(2, 0), 2);
        assert_eq!(grid.count_neighbors(2, 1), 3);
        assert_eq!(grid.count_neighbors(2, 2), 3);
    }
}
