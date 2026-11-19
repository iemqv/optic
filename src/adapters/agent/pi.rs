use std::io;
use std::process::Command;

use super::AgentClient;

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Stdio};

pub(super) struct PiClient;

impl PiClient {
    fn request_state(child: &mut Child) -> io::Result<()> {
        let mut stdin = child.stdin.take().expect("Pi stdin should be piped");
        let stdout = child.stdout.take().expect("Pi stdout should be piped");

        stdin.write_all(b"{\"id\":\"optic-1\",\"type\":\"get_state\"}\n")?;

        for line in BufReader::new(stdout).lines() {
            let message: serde_json::Value = serde_json::from_str(&line?)?;

            if message["type"] != "response" || message["id"] != "optic-1" {
                continue;
            }

            if message["success"].as_bool() == Some(true) {
                return Ok(());
            }

            return Err(io::Error::other("Pi rejected the get_state request"));
        }

        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Pi closed stdout before replying",
        ))
    }
}

impl AgentClient for PiClient {
    fn version(&self) -> io::Result<String> {
        let output = Command::new("pi").arg("--version").output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            return Err(io::Error::other(format!(
                "pi --version failed ({}): {}",
                output.status,
                stderr.trim_end()
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }

    fn check_connection(&self) -> io::Result<()> {
        let mut child = Command::new("pi")
            .args(["--mode", "rpc", "--no-session"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let response = Self::request_state(&mut child);

        let cleanup = (|| -> io::Result<()> {
            if child.try_wait()?.is_none() {
                child.kill()?;
                child.wait()?;
            }
            Ok(())
        })();

        response?;
        cleanup
    }
}
