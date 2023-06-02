use krabmaga::*;
mod model;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::board_vis::BoardVis, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
    krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
    krabmaga::visualization::fields::number_grid_2d::BatchRender,
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

    simulate!(state, step, 10);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {

    use model::board::Board;

    let num_agents = 4;
    let dim: (u16, u16) = (40, 40);

    let state = Board::new(dim, num_agents);
    let mut app = Visualization::default()
        .with_window_dimensions(50., 50.)
        .with_simulation_dimensions(dim.0.into(), dim.1.into())
        .with_background_color(Color::WHITE)
        .with_name("Template")
        .setup::<BoardVis, Board>(BoardVis, state);
    app.add_system(DenseNumberGrid2D::batch_render);
    app.run()
}