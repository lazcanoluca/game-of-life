use std::time::Instant;

use game_of_life::{
    cell::{Cell, CellState},
    grid::Grid,
    life::LifeState,
};
use macroquad::{
    color, input, shapes,
    window::{self, Conf},
};

const CELLS_VERTICAL: usize = 20;
const CELLS_HORIZONTAL: usize = 30;
const BASE_SPEED_TICKS_OVER_SECOND: u128 = 5;

trait ToColor {
    fn to_color(&self) -> color::Color;
}

impl ToColor for CellState {
    fn to_color(&self) -> color::Color {
        match self {
            CellState::Alive => color::BLACK,
            CellState::Dead => color::WHITE,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_owned(),
        window_width: 1200,
        window_height: (1200.0 * (CELLS_VERTICAL as f64 / CELLS_HORIZONTAL as f64)) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let grid = create_grid().await;

    run_game(grid).await;
}

async fn create_grid() -> Grid {
    let blank_matrix = vec![vec![Cell::new(CellState::Dead); CELLS_VERTICAL]; CELLS_HORIZONTAL];
    let mut grid = Grid::new(&blank_matrix);

    window::clear_background(color::WHITE);

    let cell_size = window::screen_width() / grid.cols_size() as f32;

    let mut selected = (0, 0);
    window::clear_background(color::WHITE);

    loop {
        for i in 0..grid.cells.len() {
            for j in 0..grid.cells[0].len() {
                shapes::draw_rectangle(
                    i as f32 * cell_size,
                    j as f32 * cell_size,
                    cell_size,
                    cell_size,
                    grid.cells[i][j].state.to_color(),
                )
            }
        }

        shapes::draw_rectangle_lines(
            selected.1 as f32 * cell_size,
            selected.0 as f32 * cell_size,
            cell_size,
            cell_size,
            cell_size / 20.0,
            color::GRAY,
        );

        if input::is_key_pressed(input::KeyCode::Left) {
            selected.1 -= if selected.1 > 0 { 1 } else { 0 }
        }

        if input::is_key_pressed(input::KeyCode::Right) {
            selected.1 += if selected.1 < (grid.cells[0].len() - 1) {
                1
            } else {
                0
            }
        }

        if input::is_key_pressed(input::KeyCode::Up) {
            selected.0 -= if selected.0 > 0 { 1 } else { 0 }
        }

        if input::is_key_pressed(input::KeyCode::Down) {
            selected.0 += if selected.0 < (grid.cells.len() - 1) {
                1
            } else {
                0
            }
        }

        if input::is_key_pressed(input::KeyCode::Tab) {
            grid.cells[selected.1][selected.0].toggle();
        }

        if input::is_key_pressed(input::KeyCode::R) {
            grid.randomize();
        }

        if input::is_key_pressed(input::KeyCode::Enter) {
            println!("Begin!");
            break;
        }

        window::next_frame().await;
    }

    grid
}

async fn run_game(grid: Grid) {
    let mut state = LifeState::new(&grid);
    let mut start = Instant::now();
    let mut ticks_per_second = BASE_SPEED_TICKS_OVER_SECOND;
    let mut paused = false;

    window::clear_background(color::WHITE);

    let cell_size = window::screen_width() / grid.cols_size() as f32;

    loop {
        for i in 0..state.grid.cells.len() {
            for j in 0..state.grid.cells[i].len() {
                shapes::draw_rectangle(
                    i as f32 * cell_size,
                    j as f32 * cell_size,
                    cell_size,
                    cell_size,
                    state.grid.cells[i][j].state.to_color(),
                )
            }
        }

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

        if !paused && start.elapsed().as_millis() > 1000 / ticks_per_second {
            state = state.step();
            start = Instant::now();
        }

        window::next_frame().await;
    }
}
