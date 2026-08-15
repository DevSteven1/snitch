use std::sync::Arc;

use crate::log::LogStore;

use super::{Command, ExitStatus, ProcessId};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SupervisorError {
    #[error("failed to start process: {0}")]
    StartFailed(String),
    #[error("failed to wait for process: {0}")]
    WaitFailed(String),
    #[error("process not found")]
    ProcessNotFound,
}

pub trait ProcessSupervisor {
    fn start(
        &mut self,
        command: &Command,
        log_store: Arc<dyn LogStore>,
    ) -> Result<ProcessId, SupervisorError>;

    fn wait(&mut self, process_id: ProcessId) -> Result<ExitStatus, SupervisorError>;
}
