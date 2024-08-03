use crate::grid::Grid;

#[derive(Clone, Debug)]
pub struct LifeState {
    pub grid: Grid,
}

impl LifeState {
    pub fn new(grid: &Grid) -> Self {
        Self { grid: grid.clone() }
    }

    pub fn step(&mut self) -> LifeState {
        self.grid.update();

        Self {
            grid: self.grid.clone(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::cell::CellState;

//     use super::*;

// #[test]
// fn test_new_life_state() {
//     let seed = vec![
//         vec![Cell::new(CellState::Alive), Cell::new(CellState::Dead)],
//         vec![Cell::new(CellState::Dead), Cell::new(CellState::Alive)],
//     ];
//     let life_state = LifeState::new(&Grid::new(&seed));
//     assert_eq!(life_state.grid.cells.len(), 2);
//     assert_eq!(life_state.grid.cells[0].len(), 2);
//     assert_eq!(life_state.grid.cells[0][0].state, CellState::Alive);
//     assert_eq!(life_state.grid.cells[0][1].state, CellState::Dead);
//     assert_eq!(life_state.grid.cells[1][0].state, CellState::Dead);
//     assert_eq!(life_state.grid.cells[1][1].state, CellState::Alive);
// }

// #[test]
// fn test_single_dead_cell_remains_dead() {
//     let seed = vec![vec![Cell::new(CellState::Dead)]];
//     let life_state = LifeState::new(&Grid::new(&seed));
//     let next_state = life_state.step();
//     assert_eq!(next_state.grid.cells[0][0].state, CellState::Dead);
// }

// #[test]
// fn test_single_dead_cell_0_neighbors_remains_dead() {
//     let seed = vec![vec![Cell::new(CellState::Dead); 3]; 3];
//     let life_state = LifeState::new(&Grid::new(&seed));
//     let next_state = life_state.step();
//     assert_eq!(next_state.grid.cells[1][1].state, CellState::Dead);
// }

// #[test]
// fn test_single_dead_cell_1_neighbors_remains_dead() {
//     let seed = vec![
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Alive),
//             Cell::new(CellState::Dead),
//         ],
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//         ],
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//         ],
//     ];
//     let life_state = LifeState::new(&Grid::new(&seed));
//     let next_state = life_state.step();
//     assert_eq!(next_state.grid.cells[1][1].state, CellState::Dead);
// }

// #[test]
// fn test_single_dead_cell_2_neighbors_remains_dead() {
//     let seed = vec![
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Alive),
//             Cell::new(CellState::Dead),
//         ],
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Alive),
//         ],
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//         ],
//     ];
//     let life_state = LifeState::new(&Grid::new(&seed));
//     let next_state = life_state.step();
//     assert_eq!(next_state.grid.cells[1][1].state, CellState::Dead);
// }

// #[test]
// fn test_single_dead_cell_3_neighbors_becomes_alive() {
//     let seed = vec![
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Alive),
//             Cell::new(CellState::Dead),
//         ],
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Alive),
//         ],
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Alive),
//         ],
//     ];
//     let life_state = LifeState::new(&Grid::new(&seed));
//     let next_state = life_state.step();
//     assert_eq!(next_state.grid.cells[1][1].state, CellState::Alive);
// }

// #[test]
// fn test_single_dead_cell_4_neighbors_remains_dead() {
//     let seed = vec![
//         vec![
//             Cell::new(CellState::Alive),
//             Cell::new(CellState::Alive),
//             Cell::new(CellState::Dead),
//         ],
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Dead),
//         ],
//         vec![
//             Cell::new(CellState::Dead),
//             Cell::new(CellState::Alive),
//             Cell::new(CellState::Alive),
//         ],
//     ];
//     let life_state = LifeState::new(&Grid::new(&seed));
//     let next_state = life_state.step();
//     assert_eq!(next_state.grid.cells[1][1].state, CellState::Dead);
// }

//     #[test]
//     fn test_single_live_cell_0_neighbors_dies() {
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//         ];
//         let life_state = LifeState::new(seed);
//         let next_state = life_state.step();
//         assert_eq!(next_state.grid[1][1].state, CellState::Dead);
//     }

//     #[test]
//     fn test_single_live_cell_1_neighbor_dies() {
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//         ];
//         let life_state = LifeState::new(seed);
//         let next_state = life_state.step();
//         assert_eq!(next_state.grid[1][1].state, CellState::Dead);
//     }

//     #[test]
//     fn test_single_live_cell_2_neighbors_remains_alive() {
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//         ];
//         let life_state = LifeState::new(seed);
//         let next_state = life_state.step();
//         assert_eq!(next_state.grid[1][1].state, CellState::Alive);
//     }

//     #[test]
//     fn test_single_live_cell_3_neighbors_remains_alive() {
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//             ],
//         ];
//         let life_state = LifeState::new(seed);
//         let next_state = life_state.step();
//         assert_eq!(next_state.grid[1][1].state, CellState::Alive);
//     }

//     #[test]
//     fn test_single_live_cell_4_neighbors_dies() {
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//             ],
//         ];
//         let life_state = LifeState::new(seed);
//         let next_state = life_state.step();
//         assert_eq!(next_state.grid[1][1].state, CellState::Dead);
//     }

//     #[test]
//     fn test_single_live_cell_5_neighbors_dies() {
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//             ],
//         ];
//         let life_state = LifeState::new(seed);
//         let next_state = life_state.step();
//         assert_eq!(next_state.grid[1][1].state, CellState::Dead);
//     }

//     #[test]
//     fn test_block() {
//         // https://conwaylife.com/wiki/Block
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//         ];
//         let mut state = LifeState::new(seed);
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                 ],
//             ]
//         );
//     }

//     #[test]
//     fn test_blinker_horizontal() {
//         // https://conwaylife.com/wiki/Blinker
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//             ],
//         ];
//         let mut state = LifeState::new(seed);
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//             ]
//         );
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                 ],
//             ]
//         );
//     }

//     #[test]
//     fn test_blinker_vertical() {
//         // https://conwaylife.com/wiki/Blinker
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//         ];
//         let mut state = LifeState::new(seed);
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                 ],
//             ]
//         );
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//             ]
//         );
//     }

//     #[test]
//     fn test_glider() {
//         // https://conwaylife.com/wiki/Glider
//         let seed = vec![
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![
//                 Cell::new(CellState::Dead),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Alive),
//                 Cell::new(CellState::Dead),
//             ],
//             vec![Cell::new(CellState::Dead); 4],
//         ];
//         let mut state = LifeState::new(seed);
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![Cell::new(CellState::Dead); 4],
//             ]
//         );
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![Cell::new(CellState::Dead); 4],
//             ]
//         );
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![Cell::new(CellState::Dead); 4],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                 ],
//             ]
//         );
//         state = state.step();
//         assert_eq!(
//             state.grid,
//             vec![
//                 vec![Cell::new(CellState::Dead); 4],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                 ],
//                 vec![
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Dead),
//                     Cell::new(CellState::Alive),
//                     Cell::new(CellState::Alive),
//                 ],
//             ]
//         )
//     }
// }
