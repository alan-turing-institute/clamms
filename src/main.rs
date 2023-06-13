use krabmaga::*;
mod config;
mod model;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::board_vis::BoardVis, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    use crate::model::board::Board;

    let step = 100;
    let num_agents = 4;
    let dim: (u16, u16) = (10, 10);

    let state = Board::new(dim, num_agents);

    simulate!(state, step, 10, false);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    use model::board::Board;

    let num_agents = 4;
    let dim: (u16, u16) = (20, 20);

    let state = Board::new(dim, num_agents);
    Visualization::default()
        // .with_window_dimensions((dim.0+2).into(), (dim.1+2).into())
        .with_simulation_dimensions((dim.0 + 2).into(), (dim.1 + 2).into())
        .with_background_color(Color::GRAY)
        .with_name("Template")
        .start::<BoardVis, Board>(BoardVis, state);
}
