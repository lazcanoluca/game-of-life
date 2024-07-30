#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new(state: CellState) -> Self {
        Self { state }
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
