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
    use krabmaga::engine::{schedule::Schedule, state::State};

    let step = 100;
    let num_agents = 4;
    let dim: (u16, u16) = (10, 10);

    let mut state = Board::new(dim, num_agents);

    // Use simulate
    // simulate!(state, step, 10, false);

    // Use scheduler and run directly once
    let mut schedule: Schedule = Schedule::new();
    let state = state.as_state_mut();
    state.init(&mut schedule);
    for i in 0..step {
        println!("Step: {i}");
        schedule.step(state);
    }
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
