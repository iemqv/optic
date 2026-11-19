use std::io;

pub(crate) trait AgentGateway {
    fn version(&self) -> io::Result<String>;
    fn check_connection(&self) -> io::Result<()>;
}
