use snitch_domain::process::{ExitStatus, ProcessId, ProcessSupervisor, SupervisorError};

pub struct WaitForSupervisedProcess<'a> {
    supervisor: &'a mut dyn ProcessSupervisor,
}

impl<'a> WaitForSupervisedProcess<'a> {
    pub fn new(supervisor: &'a mut dyn ProcessSupervisor) -> Self {
        Self { supervisor }
    }

    pub fn execute(&mut self, process_id: ProcessId) -> Result<ExitStatus, SupervisorError> {
        self.supervisor.wait(process_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snitch_domain::log::LogStore;
    use std::sync::Arc;

    #[derive(Default)]
    struct FakeSupervisor {
        should_fail: bool,
        waited_on: Vec<ProcessId>,
    }

    impl ProcessSupervisor for FakeSupervisor {
        fn start(
            &mut self,
            _command: &snitch_domain::process::Command,
            _log_store: Arc<dyn LogStore>,
        ) -> Result<ProcessId, SupervisorError> {
            unreachable!("not exercised by these tests")
        }

        fn wait(&mut self, process_id: ProcessId) -> Result<ExitStatus, SupervisorError> {
            if self.should_fail {
                return Err(SupervisorError::ProcessNotFound);
            }
            self.waited_on.push(process_id);
            Ok(ExitStatus::new(Some(0)))
        }
    }

    #[test]
    fn waits_on_the_given_process_through_the_supervisor() {
        let mut supervisor = FakeSupervisor::default();
        let mut use_case = WaitForSupervisedProcess::new(&mut supervisor);

        let status = use_case.execute(ProcessId::new(7)).unwrap();

        assert!(status.success());
        assert_eq!(supervisor.waited_on, vec![ProcessId::new(7)]);
    }

    #[test]
    fn propagates_a_supervisor_failure() {
        let mut supervisor = FakeSupervisor {
            should_fail: true,
            ..Default::default()
        };
        let mut use_case = WaitForSupervisedProcess::new(&mut supervisor);

        let result = use_case.execute(ProcessId::new(7));

        assert_eq!(result, Err(SupervisorError::ProcessNotFound));
    }
}
