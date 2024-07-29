mod gui;
mod life;

use gui::View;
use life::LifeState;
use std::time::Instant;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let life = LifeState::new_with_random_state();

    let gui = View::new();

    let mut start = Instant::now();

    loop {
        gui.draw(&life).await;

        if start.elapsed().as_secs() > 1 {
            life.step();
            start = Instant::now();
        }
    }
}
