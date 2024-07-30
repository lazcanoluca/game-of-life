use std::time::Instant;

use game_of_life::{
    cell::{Cell, CellState},
    gui::View,
    life::LifeState,
};

// https://conwaylife.com/wiki/Blinker
#[macroquad::main("Conway's Game of Life - Blinker")]
async fn main() {
    let blinker = vec![
        vec![Cell::new(CellState::Dead); 5],
        vec![
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Alive),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
        ],
        vec![
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Alive),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
        ],
        vec![
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Alive),
            Cell::new(CellState::Dead),
            Cell::new(CellState::Dead),
        ],
        vec![Cell::new(CellState::Dead); 5],
    ];
    let mut state = LifeState::new(blinker);

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
