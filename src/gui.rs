use macroquad::{
    color::{self, Color},
    shapes, window,
};

use crate::{cell::CellState, life::LifeState};

impl From<CellState> for Color {
    fn from(value: CellState) -> Self {
        match value {
            CellState::Alive => color::BLACK,
            CellState::Dead => color::WHITE,
        }
    }
}

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn draw(&self, state: &LifeState) {
        window::clear_background(color::WHITE);

        let cell_size = f32::min(window::screen_width(), window::screen_height())
            / usize::max(state.grid.len(), state.grid[0].len()) as f32;

        for i in 0..state.grid.len() {
            for j in 0..state.grid[i].len() {
                shapes::draw_rectangle(
                    i as f32 * cell_size,
                    j as f32 * cell_size,
                    cell_size,
                    cell_size,
                    state.grid[i][j].state.into(),
                )
            }
        }

        window::next_frame().await;
    }
}
