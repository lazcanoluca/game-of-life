use rand::seq::IteratorRandom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new(state: CellState) -> Self {
        Self { state }
    }

    pub fn new_with_random_state() -> Self {
        Self {
            state: [CellState::Dead, CellState::Alive]
                .iter()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone(),
        }
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            CellState::Dead => CellState::Alive,
            CellState::Alive => CellState::Dead,
        }
    }

    pub fn kill(&mut self) {
        self.state = CellState::Dead;
    }

    pub fn resurrect(&mut self) {
        self.state = CellState::Alive;
    }
}

// TODO: CellIndex / CellCoordinate struct

// TODO: maquina de estados para las transiciones

impl CellState {
    // pub fn toggle(&mut self) {
    //     *self = match self {
    //         CellState::Dead => CellState::Alive,
    //         CellState::Alive => CellState::Dead,
    //     }
    // }
}

// impl Iterator for CellState {
//     type Item = CellState;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// pub struct Life {
//     state: LifeState
// }

// struct Grid {}

// TODO: cell method for counting cells in some state around it

#[derive(Clone, Debug)]
pub struct LifeState {
    pub grid: Vec<Vec<Cell>>,
}

impl LifeState {
    pub fn new(seed: Vec<Vec<Cell>>) -> Self {
        Self { grid: seed }
    }

    pub fn new_with_random_seed() -> Self {
        Self {
            grid: (0..100)
                .map(|_| (0..100).map(|_| Cell::new_with_random_state()).collect())
                .collect(),
        }
    }

    pub fn step(&self) -> LifeState {
        // todo!("Apply rules and return state.")

        let w = self.grid.len();
        let h = self.grid[0].len();

        let mut new_grid = self.grid.clone();

        for x in 0..w as i32 {
            for y in 0..h as i32 {
                let mut neighbors_count = 0;

                for i in -1i32..=1 {
                    for j in -1i32..=1 {
                        // Out of bounds.
                        if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
                            continue;
                        }
                        // TODO: if self.grid.is_out_of_bounds
                        // cell itself
                        if i == 0 && j == 0 {
                            continue;
                        }

                        let neighbor = self.grid[(x + i) as usize][(j + y) as usize].state;

                        if neighbor == CellState::Alive {
                            neighbors_count += 1;
                            // println!("Alive!");
                        }
                    }
                }

                let current_cell = &mut new_grid[x as usize][y as usize];
                current_cell.state = match (current_cell.state, neighbors_count) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (CellState::Alive, x) if x < 2 => {
                        // println!("Kill!");
                        // println!("killing: {:?}", current_cell);
                        // current_cell.kill();
                        // println!("{:?}", current_cell);
                        // println!("\n");
                        CellState::Dead
                    }
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    // (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (CellState::Alive, x) if x > 3 => {
                        // println!("Kill!");
                        // println!("killing: {:?}", current_cell);
                        // current_cell.kill();
                        // println!("{:?}", current_cell);
                        // println!("\n");
                        CellState::Dead
                    }
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (CellState::Dead, 3) => {
                        // println!("Resurrect!");
                        // println!("aliving: {:?}", current_cell);
                        // current_cell.resurrect();
                        // println!("{:?}", current_cell);
                        // println!("\n");
                        CellState::Alive
                    }
                    // All other cells remain in the same state.
                    // (otherwise, _) => otherwise,
                    (otherwise, _) => otherwise,
                };
            }
        }

        // println!("{:?}", new_grid);
        LifeState { grid: new_grid }
    }
}

// TODO: iterator for Grid
