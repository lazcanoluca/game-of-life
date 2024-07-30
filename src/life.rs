use crate::cell::{Cell, CellState};

#[derive(Clone, Debug)]
pub struct LifeState {
    pub grid: Vec<Vec<Cell>>,
}

impl LifeState {
    pub fn new(seed: Vec<Vec<Cell>>) -> Self {
        Self { grid: seed }
    }

    fn count_neighbors(&self, row_index: u32, col_index: u32) -> u32 {
        let mut neighbors_count = 0;

        for j in -1i32..=1 {
            for i in -1i32..=1 {
                // Out of bounds.
                if col_index as i32 + j < 0
                    || col_index as i32 + j >= self.grid.len() as i32
                    || row_index as i32 + i < 0
                    || row_index as i32 + i >= self.grid[0].len() as i32
                {
                    continue;
                }

                // Current cell.
                if i == 0 && j == 0 {
                    continue;
                }

                let neighbor = self.grid[(row_index as i32 + i) as usize]
                    [(j + col_index as i32) as usize]
                    .state;

                if neighbor == CellState::Alive {
                    neighbors_count += 1;
                }
            }
        }

        neighbors_count
    }

    pub fn step(&self) -> LifeState {
        let mut new_grid = self.grid.clone();

        for row in 0..self.grid.len() as u32 {
            for col in 0..self.grid[0].len() as u32 {
                let neighbors_count = self.count_neighbors(row, col);

                let current_cell = &mut new_grid[row as usize][col as usize];
                current_cell.state = match (current_cell.state, neighbors_count) {
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
                };
            }
        }

        Self { grid: new_grid }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::CellState;

    use super::*;

    #[test]
    fn test_new_life_state() {
        let seed = vec![
            vec![Cell::new(CellState::Alive), Cell::new(CellState::Dead)],
            vec![Cell::new(CellState::Dead), Cell::new(CellState::Alive)],
        ];
        let life_state = LifeState::new(seed);
        assert_eq!(life_state.grid.len(), 2);
        assert_eq!(life_state.grid[0].len(), 2);
        assert_eq!(life_state.grid[0][0].state, CellState::Alive);
        assert_eq!(life_state.grid[0][1].state, CellState::Dead);
        assert_eq!(life_state.grid[1][0].state, CellState::Dead);
        assert_eq!(life_state.grid[1][1].state, CellState::Alive);
    }

    #[test]
    fn test_count_neigbors() {
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
        let life_state = LifeState::new(seed);
        assert_eq!(life_state.count_neighbors(0, 0), 1);
        assert_eq!(life_state.count_neighbors(0, 1), 3);
        assert_eq!(life_state.count_neighbors(0, 2), 2);
        assert_eq!(life_state.count_neighbors(1, 0), 4);
        assert_eq!(life_state.count_neighbors(1, 1), 4);
        assert_eq!(life_state.count_neighbors(1, 2), 2);
        assert_eq!(life_state.count_neighbors(2, 0), 2);
        assert_eq!(life_state.count_neighbors(2, 1), 3);
        assert_eq!(life_state.count_neighbors(2, 2), 3);
    }

    #[test]
    fn test_single_dead_cell_remains_dead() {
        let seed = vec![vec![Cell::new(CellState::Dead)]];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[0][0].state, CellState::Dead);
    }

    #[test]
    fn test_single_dead_cell_0_neighbors_remains_dead() {
        let seed = vec![vec![Cell::new(CellState::Dead); 3]; 3];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Dead);
    }

    #[test]
    fn test_single_dead_cell_1_neighbors_remains_dead() {
        let seed = vec![
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Dead);
    }

    #[test]
    fn test_single_dead_cell_2_neighbors_remains_dead() {
        let seed = vec![
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Dead);
    }

    #[test]
    fn test_single_dead_cell_3_neighbors_becomes_alive() {
        let seed = vec![
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Alive);
    }

    #[test]
    fn test_single_dead_cell_4_neighbors_remains_dead() {
        let seed = vec![
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Dead);
    }

    #[test]
    fn test_single_live_cell_0_neighbors_dies() {
        let seed = vec![
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Dead);
    }

    #[test]
    fn test_single_live_cell_1_neighbor_dies() {
        let seed = vec![
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Dead);
    }

    #[test]
    fn test_single_live_cell_2_neighbors_remains_alive() {
        let seed = vec![
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Alive);
    }

    #[test]
    fn test_single_live_cell_3_neighbors_remains_alive() {
        let seed = vec![
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Alive);
    }

    #[test]
    fn test_single_live_cell_4_neighbors_dies() {
        let seed = vec![
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Dead);
    }

    #[test]
    fn test_single_live_cell_5_neighbors_dies() {
        let seed = vec![
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
            ],
        ];
        let life_state = LifeState::new(seed);
        let next_state = life_state.step();
        assert_eq!(next_state.grid[1][1].state, CellState::Dead);
    }

    #[test]
    fn test_block() {
        // https://conwaylife.com/wiki/Block
        let seed = vec![
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
        ];
        let mut state = LifeState::new(seed);
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                ],
            ]
        );
    }

    #[test]
    fn test_blinker_horizontal() {
        // https://conwaylife.com/wiki/Blinker
        let seed = vec![
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
            ],
        ];
        let mut state = LifeState::new(seed);
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
            ]
        );
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                ],
            ]
        );
    }

    #[test]
    fn test_blinker_vertical() {
        // https://conwaylife.com/wiki/Blinker
        let seed = vec![
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
        ];
        let mut state = LifeState::new(seed);
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                ],
            ]
        );
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
            ]
        );
    }

    #[test]
    fn test_glider() {
        // https://conwaylife.com/wiki/Glider
        let seed = vec![
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![
                Cell::new(CellState::Dead),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Alive),
                Cell::new(CellState::Dead),
            ],
            vec![Cell::new(CellState::Dead); 4],
        ];
        let mut state = LifeState::new(seed);
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
                vec![Cell::new(CellState::Dead); 4],
            ]
        );
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                ],
                vec![Cell::new(CellState::Dead); 4],
            ]
        );
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![Cell::new(CellState::Dead); 4],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                ],
            ]
        );
        state = state.step();
        assert_eq!(
            state.grid,
            vec![
                vec![Cell::new(CellState::Dead); 4],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                ],
                vec![
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Dead),
                    Cell::new(CellState::Alive),
                    Cell::new(CellState::Alive),
                ],
            ]
        )
    }
}
