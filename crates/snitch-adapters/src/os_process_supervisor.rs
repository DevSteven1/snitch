use std::process::Command as StdCommand;

use snitch_domain::process::{Command, ProcessId, ProcessSupervisor, SupervisorError};

#[derive(Default)]
pub struct OsProcessSupervisor;

impl OsProcessSupervisor {
    pub fn new() -> Self {
        Self
    }
}

impl ProcessSupervisor for OsProcessSupervisor {
    fn start(&mut self, command: &Command) -> Result<ProcessId, SupervisorError> {
        StdCommand::new(command.program())
            .args(command.args())
            .spawn()
            .map(|child| ProcessId::new(child.id()))
            .map_err(|error| SupervisorError::StartFailed(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    fn exiting_command() -> Command {
        Command::new("cmd", vec!["/C".to_string(), "exit".to_string(), "0".to_string()]).unwrap()
    }

    #[cfg(not(windows))]
    fn exiting_command() -> Command {
        Command::new("true", vec![]).unwrap()
    }

    #[test]
    fn spawns_a_real_process_and_returns_its_id() {
        let mut supervisor = OsProcessSupervisor::new();
        let command = exiting_command();

        let process_id = supervisor.start(&command).unwrap();

        assert!(process_id.value() > 0);
    }

    #[test]
    fn fails_to_start_a_command_that_does_not_exist() {
        let mut supervisor = OsProcessSupervisor::new();
        let command = Command::new("snitch-command-that-does-not-exist", vec![]).unwrap();

        let result = supervisor.start(&command);

        assert!(result.is_err());
    }
}
