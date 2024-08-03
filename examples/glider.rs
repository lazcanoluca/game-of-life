use std::time::Instant;

use game_of_life::{
    cell::{Cell, CellState},
    grid::Grid,
    gui::View,
    life::LifeState,
};

// https://conwaylife.com/wiki/Glider
#[macroquad::main("Conway's Game of Life - Glider")]
async fn main() {
    let glider = vec![
        vec![
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Alive),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
        ],
        vec![
            Cell::new(CellState::Alive),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Alive),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
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
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
        ],
        vec![Cell::new(CellState::Dead); 10],
        vec![Cell::new(CellState::Dead); 10],
        vec![Cell::new(CellState::Dead); 10],
        vec![Cell::new(CellState::Dead); 10],
        vec![Cell::new(CellState::Dead); 10],
        vec![Cell::new(CellState::Dead); 10],
        vec![Cell::new(CellState::Dead); 10],
    ];
    let mut state = LifeState::new(&Grid::new(&glider));

    let gui = View::new();

    let mut start = Instant::now();

    loop {
        gui.draw(&state).await;

        if start.elapsed().as_millis() > 500 {
            state = state.step();
            start = Instant::now();
        }
    }
}
