use rand::seq::IteratorRandom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

impl CellState {
    pub fn iter() -> impl Iterator<Item = CellState> {
        [CellState::Dead, CellState::Alive].into_iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new(state: CellState) -> Self {
        Self { state }
    }

    pub fn new_with_random_state() -> Self {
        Self {
            state: CellState::iter()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone(),
        }
    }

    pub fn toggle(&mut self) {
        self.state = if self.state == CellState::Alive {
            CellState::Dead
        } else {
            CellState::Alive
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cell_dead() {
        let cell = Cell::new(CellState::Dead);
        assert_eq!(cell.state, CellState::Dead);
    }

    #[test]
    fn test_new_cell_alive() {
        let cell = Cell::new(CellState::Alive);
        assert_eq!(cell.state, CellState::Alive);
    }
}
