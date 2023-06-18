use std::collections::BTreeMap;

use krabmaga::{engine::location::Int2D, hashbrown::HashSet, *};
use model::environment::Resource;
mod config;
mod model;
use crate::model::{
    action::Action,
    agent_state::{AgentStateItems, InvLevel},
    board::Board,
    forager::Forager,
    tabular_rl::SARSAModel,
};
use strum::IntoEnumIterator;

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
    use crate::{config::core_config, model::board::Board};
    use krabmaga::{
        engine::{location::Int2D, schedule::Schedule, state::State},
        hashbrown::HashSet,
    };

    let seed = core_config().world.RANDOM_SEED;
    let n_steps = core_config().world.N_STEPS;
    let num_agents = core_config().world.N_AGENTS;
    let dim: (u16, u16) = (10, 10);

    let mut model = SARSAModel::new(
        (0..num_agents).map(|n| n.into()).collect(),
        AgentStateItems::iter().collect::<Vec<AgentStateItems>>(),
        InvLevel::iter().collect::<Vec<InvLevel>>(),
        Action::iter().collect::<Vec<Action>>(),
    );

    // let mut board = Board::new_with_seed(dim, num_agents, seed, model);
    let mut board = if let Some(file_name) = &core_config().world.RESOURCE_LOCATIONS_FILE {
        Board::new_with_seed_resources(dim, num_agents, seed, &file_name, model)
    } else {
        Board::new_with_seed(dim, num_agents, seed, model)
    };
    // Use simulate
    // simulate!(state, step, 10, false);

    // setup RL model

    // Use scheduler and run directly once
    let mut schedule: Schedule = Schedule::new();
    // let state = board.as_state_mut();
    board.init(&mut schedule);
    for i in 0..n_steps {
        println!("Step: {i}");
        schedule.step(&mut board);
    }

    // // Open output file and write history
    // let mut f = File::create("output.json").unwrap();
    // writeln!(
    //     f,
    //     "{}",
    //     serde_json::to_string_pretty(&board.agent_histories).unwrap()
    // )
    // .unwrap();
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    use config::core_config;
    use krabmaga::bevy::prelude::{App, Query, World};
    use model::board::Board;

    let num_agents = core_config().world.N_AGENTS;
    let seed = core_config().world.RANDOM_SEED;
    let dim: (u16, u16) = (core_config().world.WIDTH, core_config().world.HEIGHT);

    let mut model = SARSAModel::new(
        (0..num_agents).map(|n| n.into()).collect(),
        AgentStateItems::iter().collect::<Vec<AgentStateItems>>(),
        InvLevel::iter().collect::<Vec<InvLevel>>(),
        Action::iter().collect::<Vec<Action>>(),
    );

    let state = if let Some(file_name) = &core_config().world.RESOURCE_LOCATIONS_FILE {
        Board::new_with_seed_resources(dim, num_agents, seed, &file_name, model)
    } else {
        Board::new_with_seed(dim, num_agents, seed, model)
    };
    use crate::bevy::prelude::ResMut;
    use krabmaga::engine::state::State;
    use krabmaga::visualization::visualization_state::VisualizationState;
    use krabmaga::visualization::wrappers::ActiveState;
    // fn runner(mut app: App) {
    //     for i in 0..100 {
    //         app.update();
    //     }
    // }
    Visualization::default()
        // .with_window_dimensions((dim.0+2).into(), (dim.1+2).into())
        .with_simulation_dimensions((dim.0 + 1).into(), (dim.1 + 1).into())
        .with_background_color(Color::GRAY)
        .with_name("CLAMMs")
        .setup::<BoardVis, Board>(BoardVis, state)
        // .set_runner(runner)
        .run();
}
