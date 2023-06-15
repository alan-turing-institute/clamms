use krabmaga::*;
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
    use krabmaga::engine::{schedule::Schedule, state::State};

    let seed = 0;
    let step = 1000;
    let num_agents = 4;
    let dim: (u16, u16) = (10, 10);

    let mut model = SARSAModel::new(
        (0..num_agents).map(|n| n.into()).collect(),
        AgentStateItems::iter().collect::<Vec<AgentStateItems>>(),
        InvLevel::iter().collect::<Vec<InvLevel>>(),
        Action::iter().collect::<Vec<Action>>(),
    );

    let mut board = Board::new_with_seed(dim, num_agents, seed, model);

    // Use simulate
    // simulate!(state, step, 10, false);

    // setup RL model

    // Use scheduler and run directly once
    let mut schedule: Schedule = Schedule::new();
    // let state = board.as_state_mut();
    board.init(&mut schedule);
    for i in 0..step {
        // println!("Step: {i}");
        schedule.step(&mut board);
        board.model.step(i, &board.agent_histories);
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
    use model::board::Board;

    let num_agents = 1;
    let dim: (u16, u16) = (20, 20);

    let mut model = SARSAModel::new(
        (0..num_agents).map(|n| n.into()).collect(),
        AgentStateItems::iter().collect::<Vec<AgentStateItems>>(),
        InvLevel::iter().collect::<Vec<InvLevel>>(),
        Action::iter().collect::<Vec<Action>>(),
    );

    let state = Board::new(dim, num_agents, model);
    Visualization::default()
        // .with_window_dimensions((dim.0+2).into(), (dim.1+2).into())
        .with_simulation_dimensions((dim.0 + 2).into(), (dim.1 + 2).into())
        .with_background_color(Color::GRAY)
        .with_name("Template")
        .start::<BoardVis, Board>(BoardVis, state);
}
