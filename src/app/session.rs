use super::agent::AgentGateway;
use std::io;

pub(crate) struct SessionService<G: AgentGateway> {
    agent: G,
}

impl<G: AgentGateway> SessionService<G> {
    pub(crate) fn new(agent: G) -> Self {
        Self { agent }
    }

    pub(crate) fn agent_version(&self) -> io::Result<String> {
        self.agent.version()
    }

    pub(crate) fn check_connection(&self) -> io::Result<()> {
        self.agent.check_connection()
    }
}
