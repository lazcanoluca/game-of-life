use std::time::Instant;

use game_of_life::{gui::View, life::LifeState};

const CELLS_VERTICAL: u32 = 100;
const CELLS_HORIZONTAL: u32 = 100;
const TIME_STEP_MILLIS: u128 = 250;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut state = LifeState::new_with_random_seed(CELLS_HORIZONTAL, CELLS_VERTICAL);

    let gui = View::new();

    let mut start = Instant::now();

    loop {
        gui.draw(&state).await;

        if start.elapsed().as_millis() > TIME_STEP_MILLIS {
            state = state.step();
            start = Instant::now();
        }
    }
}
