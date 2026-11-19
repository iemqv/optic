mod pi;
use self::pi::PiClient;
use crate::app::agent::AgentGateway;
use std::io;

pub(crate) enum AgentType {
    Pi,
}

trait AgentClient {
    fn version(&self) -> io::Result<String>;
    fn check_connection(&self) -> io::Result<()>;
}

pub(crate) struct AgentAdapter {
    client: Box<dyn AgentClient>,
}

impl AgentAdapter {
    pub(crate) fn new(agent_type: AgentType) -> Self {
        let client: Box<dyn AgentClient> = match agent_type {
            AgentType::Pi => Box::new(PiClient),
        };
        Self { client }
    }
}
impl AgentGateway for AgentAdapter {
    fn version(&self) -> io::Result<String> {
        self.client.version()
    }

    fn check_connection(&self) -> io::Result<()> {
        self.client.check_connection()
    }
}
