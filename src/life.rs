use rand::seq::IteratorRandom;

#[derive(Clone, Copy, Debug)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Debug)]
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
}

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

#[derive(Clone, Debug)]
pub struct LifeState {
    pub grid: Vec<Vec<Cell>>,
}

impl LifeState {
    pub fn new(initial_state: Vec<Vec<Cell>>) -> Self {
        Self {
            grid: initial_state,
        }
    }

    pub fn new_with_random_state() -> Self {
        Self {
            grid: (0..20)
                .map(|_| (0..20).map(|_| Cell::new_with_random_state()).collect())
                .collect(),
        }
    }

    pub fn step(&self) -> Self {
        todo!("Apply rules and return state.")
    }
}
