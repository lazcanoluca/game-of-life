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
