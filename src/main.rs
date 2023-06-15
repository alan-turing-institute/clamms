use std::collections::BTreeMap;

use krabmaga::{engine::location::Int2D, hashbrown::HashSet, *};
use model::environment::Resource;
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
    use krabmaga::{
        engine::{location::Int2D, schedule::Schedule, state::State},
        hashbrown::HashSet,
    };

    let seed = 0;
    let step = 100;
    let num_agents = 4;
    let dim: (u16, u16) = (10, 10);

    let mut board = Board::new_with_seed(dim, num_agents, seed);
    // Use simulate
    // simulate!(state, step, 10, false);

    // Use scheduler and run directly once
    let mut schedule: Schedule = Schedule::new();
    let state = board.as_state_mut();
    state.init(&mut schedule);
    for i in 0..step {
        println!("Step: {i}");
        schedule.step(state);
    }
    // Open output file and write history
    let mut f = File::create("output.json").unwrap();
    writeln!(
        f,
        "{}",
        serde_json::to_string_pretty(&board.agent_histories).unwrap()
    )
    .unwrap();
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    use config::core_config;
    use model::board::Board;

    let num_agents = core_config().world.N_AGENTS;
    let seed = core_config().world.RANDOM_SEED;
    let dim: (u16, u16) = (core_config().world.WIDTH, core_config().world.HEIGHT);
    let state = if let Some(file_name) = &core_config().world.RESOURCE_LOCATIONS_FILE {
        Board::new_with_seed_resources(dim, num_agents, seed, &file_name)
    } else {
        Board::new_with_seed(dim, num_agents, seed)
    };
    Visualization::default()
        // .with_window_dimensions((dim.0+2).into(), (dim.1+2).into())
        .with_simulation_dimensions((dim.0 + 1).into(), (dim.1 + 1).into())
        .with_background_color(Color::GRAY)
        .with_name("Template")
        .start::<BoardVis, Board>(BoardVis, state);
}
