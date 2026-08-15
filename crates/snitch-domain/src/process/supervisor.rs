use super::{Command, ProcessId};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SupervisorError {
    #[error("failed to start process: {0}")]
    StartFailed(String),
}

pub trait ProcessSupervisor {
    fn start(&mut self, command: &Command) -> Result<ProcessId, SupervisorError>;
}
