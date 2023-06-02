use crate::model::walker::Walker;
use crate::model::board::Board;
use crate::visualization::walker_vis::WalkerVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone, Resource)]
pub struct BoardVis;

/// Define how the simulation should be bootstrapped. Agents should be created here.

impl VisualizationState<Board> for BoardVis {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut Board,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
        Self::generate_patches(&_state, _sprite_render_factory, _commands, _sim);
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &Board,
    ) -> Option<Box<dyn AgentRender>> {
        Some(Box::new(WalkerVis {
            id: agent.downcast_ref::<Walker>().unwrap().id,
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        match state.agents_field.get(&Walker {
            id: agent_render.get_id(),
            pos: Int2D { x:0,y:0},
        }) {
            Some(matching_agent) => Some(Box::new(matching_agent)),
            None => None,
        }
    }
}

impl BoardVis {
    fn generate_patches(
        state: &Board,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        commands: &mut Commands,
        sim: &mut SimulationDescriptor,
    ) {
        state
            .field
            .render(&mut *sprite_render_factory, commands, sim);
    }
}