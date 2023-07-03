use krabmaga::engine::agent::Agent;

/// Additional generic API for extracting agents.
pub trait AgentAPI<A: Agent> {
    fn get_agent_by_id(&self, id: &u32) -> A;
    fn get_agents(&self) -> Vec<A>;
}
