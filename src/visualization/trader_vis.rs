use crate::model::policy::Policy;
use crate::model::{board::Board, trader::Trader, action::Action};
use crate::config::action2rotation;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::prelude::{Component, Quat, Transform, Visibility};
use krabmaga::{
    engine::{agent::Agent, state::State},
    visualization::agent_render::{AgentRender, SpriteType},
};
use crate::model::forager::Forager;

#[derive(Component)]
pub struct TraderVis {
    pub(crate) id: u32,
}

impl AgentRender for TraderVis {
    /// Specify the assets to use. Swap "bird" with the file name of whatever emoji you want to use.
    /// Be sure to also copy the asset itself in the assets/emojis folder. In future, this limitation will
    /// be removed.
    fn sprite(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> SpriteType {
        SpriteType::Emoji(String::from("crab"))
    }

    /// Specify where the agent should be rendered in the window.
    fn location(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        let state = state.as_any().downcast_ref::<Board>().unwrap();
        // let agent = agent.downcast_ref::<Trader>().unwrap();
        // (agent.forager().pos.x as f32, agent.forager().pos.y as f32, 2.)
        // let pos = state.field.get_location(*agent);
        // match pos {
        //     Some(pos) => (pos.x as f32, pos.y as f32, 0.),
        //     None => (agent.pos.x as f32, agent.pos.y as f32, 0.),
        // }

        if let Some(trader) = agent.downcast_ref::<Trader>() {
            (trader.forager().pos.x as f32, trader.forager().pos.y as f32, 2.)
        } else if let Some(forager) = agent.downcast_ref::<Forager>() {
            (forager.pos.x as f32, forager.pos.y as f32, 2.)
        } else {
            panic!()
        }
    }

    /// Specify how much the texture should be scaled by. A common scale is (0.1, 0.1).
    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (0.016, 0.016)
    }

    /// Define the degrees in radians to rotate the texture by.
    fn rotation(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {

        let action: Action;
        if let Some(trader) = agent.as_any().downcast_ref::<Trader>() {
            let agent_state = trader.forager.agent_state(**_state);
            action = trader.choose_action(&agent_state);
        } else {
            let forager  = agent.as_any().downcast_ref::<Forager>().unwrap();
            let agent_state = forager.agent_state(**_state);
            action = forager.choose_action(&agent_state);
        } 

        action2rotation(action)
    }

    /// Specify the code to execute for each frame, for each agent.
    fn update(
        &mut self,
        agent: &Box<dyn Agent>,
        transform: &mut Transform,
        state: &Box<&dyn State>,
        _visible: &mut Visibility,
    ) {
        // This snippet updates the agent location, scale and rotation for each frame.
        let (loc_x, loc_y, z) = self.location(agent, state);
        let rotation = self.rotation(agent, state);
        let (scale_x, scale_y) = self.scale(agent, state);

        let translation = &mut transform.translation;
        translation.x = loc_x;
        translation.y = loc_y;
        translation.z = z;
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
        transform.rotation = Quat::from_rotation_z(rotation);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
