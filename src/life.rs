use crate::cell::Cell;

#[derive(Clone, Debug)]
pub struct LifeState {
    pub grid: Vec<Vec<Cell>>,
}

impl LifeState {
    pub fn new(seed: Vec<Vec<Cell>>) -> Self {
        Self { grid: seed }
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
}
