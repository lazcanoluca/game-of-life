mod cell;
mod gui;
mod life;

use gui::View;
// use life_old::LifeState;
use std::time::Instant;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    // let mut state = LifeState::new_with_random_seed();

    let gui = View::new();

    let mut start = Instant::now();

    loop {
        // gui.draw(&state).await;

        // if start.elapsed().as_millis() > 500 {
        //     state = state.step();
        //     start = Instant::now();
        // }
    }
}
