use std::sync::Arc;

use snitch_domain::log::LogStore;
use snitch_domain::process::{Command, ProcessId, ProcessSupervisor, SupervisorError};

pub struct StartSupervisedProcess<'a> {
    supervisor: &'a mut dyn ProcessSupervisor,
    log_store: Arc<dyn LogStore>,
}

impl<'a> StartSupervisedProcess<'a> {
    pub fn new(supervisor: &'a mut dyn ProcessSupervisor, log_store: Arc<dyn LogStore>) -> Self {
        Self {
            supervisor,
            log_store,
        }
    }

    pub fn execute(&mut self, command: &Command) -> Result<ProcessId, SupervisorError> {
        self.supervisor.start(command, self.log_store.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snitch_domain::log::LogLine;
    use snitch_domain::process::ProcessId as DomainProcessId;

    #[derive(Default)]
    struct FakeSupervisor {
        next_id: u32,
        should_fail: bool,
        started: Vec<Command>,
    }

    impl ProcessSupervisor for FakeSupervisor {
        fn start(
            &mut self,
            command: &Command,
            _log_store: Arc<dyn LogStore>,
        ) -> Result<ProcessId, SupervisorError> {
            if self.should_fail {
                return Err(SupervisorError::StartFailed("boom".to_string()));
            }
            self.next_id += 1;
            self.started.push(command.clone());
            Ok(ProcessId::new(self.next_id))
        }
    }

    #[derive(Default)]
    struct NullLogStore;

    impl LogStore for NullLogStore {
        fn append(&self, _line: LogLine) {}

        fn lines_for(&self, _process_id: DomainProcessId) -> Vec<LogLine> {
            Vec::new()
        }
    }

    #[test]
    fn starts_the_process_through_the_supervisor_and_returns_its_id() {
        let mut supervisor = FakeSupervisor::default();
        let command = Command::new("cargo", vec!["run".to_string()]).unwrap();
        let mut use_case = StartSupervisedProcess::new(&mut supervisor, Arc::new(NullLogStore));

        let result = use_case.execute(&command).unwrap();

        assert_eq!(result, ProcessId::new(1));
    }

    #[test]
    fn forwards_the_exact_command_to_the_supervisor() {
        let mut supervisor = FakeSupervisor::default();
        let command = Command::new("cargo", vec!["run".to_string()]).unwrap();
        let mut use_case = StartSupervisedProcess::new(&mut supervisor, Arc::new(NullLogStore));

        use_case.execute(&command).unwrap();

        assert_eq!(supervisor.started, vec![command]);
    }

    #[test]
    fn propagates_a_supervisor_failure() {
        let mut supervisor = FakeSupervisor {
            should_fail: true,
            ..Default::default()
        };
        let command = Command::new("cargo", vec!["run".to_string()]).unwrap();
        let mut use_case = StartSupervisedProcess::new(&mut supervisor, Arc::new(NullLogStore));

        let result = use_case.execute(&command);

        assert_eq!(
            result,
            Err(SupervisorError::StartFailed("boom".to_string()))
        );
    }
}
