use crate::model::board::{Board, Patch};
use crate::model::forager::Forager;
use crate::visualization::forager_vis::ForagerVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone, Resource)]
pub struct BoardVis;

/// Define how the simulation should be bootstrapped. Agents should be created here.

impl VisualizationState<Board> for BoardVis {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_factory: &mut AssetHandleFactoryResource,
        _state: &mut Board,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
        _state.resource_grid.update();
        DenseGrid2D::<Patch>::init_graphics_grid(_sprite_factory, _commands, _state);
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &Board,
    ) -> Option<Box<dyn AgentRender>> {
        Some(Box::new(ForagerVis {
            id: agent.downcast_ref::<Forager>().unwrap().id(),
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        match state.forager_grid.get(&Forager::dummy(agent_render.get_id())) {
            Some(matching_agent) => Some(Box::new(matching_agent)),
            None => None,
        }
    }
}

impl BoardVis {}
