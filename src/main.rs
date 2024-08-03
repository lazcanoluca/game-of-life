use std::time::Instant;

use game_of_life::{grid::Grid, gui::View, life::LifeState};
use macroquad::input;

const CELLS_VERTICAL: usize = 100;
const CELLS_HORIZONTAL: usize = 100;
const BASE_SPEED_TICKS_OVER_SECOND: u128 = 5;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let grid = Grid::new_with_random_seed(CELLS_HORIZONTAL, CELLS_VERTICAL);

    run_game(grid).await;
}

async fn run_game(grid: Grid) {
    let mut state = LifeState::new(&grid);

    let gui = View::new();

    let mut start = Instant::now();

    let mut ticks_per_second = BASE_SPEED_TICKS_OVER_SECOND;

    let mut paused = false;

    loop {
        gui.draw(&state).await;

        if input::is_key_pressed(input::KeyCode::Up) {
            ticks_per_second += 1;
            println!("Speed: {} ticks / second", ticks_per_second);
        }

        if input::is_key_pressed(input::KeyCode::Down) {
            ticks_per_second -= if ticks_per_second > 1 { 1 } else { 0 };
            println!("Speed: {} ticks / second", ticks_per_second);
        }

        if input::is_key_pressed(input::KeyCode::Space) {
            paused = !paused;
            println!("{}", if paused { "Paused!" } else { "Running!" })
        }

        if input::is_key_pressed(input::KeyCode::R) {
            state = LifeState::new(&grid);
            start = Instant::now();
            println!("Reset!");
        }

        if paused {
            continue;
        }

        if start.elapsed().as_millis() > 1000 / ticks_per_second {
            state = state.step();
            start = Instant::now();
        }
    }
}
