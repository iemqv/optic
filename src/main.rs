mod adapters;
mod app;
mod tui;

use adapters::agent::{AgentAdapter, AgentType};
use app::session::SessionService;
use std::process::ExitCode;

fn main() -> ExitCode {
    let adapter = AgentAdapter::new(AgentType::Pi);
    let service = SessionService::new(adapter);

    match service.agent_version() {
        Ok(version) => {
            println!("{version}");

            match service.check_connection() {
                Ok(_) => {
                    println!("Pi RPC is ready");
                }
                Err(error) => {
                    eprintln!("Failed to establish Pi RPC: {error}");
                    return ExitCode::FAILURE;
                }
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Failed to get agent version: {error}");
            ExitCode::FAILURE
        }
    }
}
