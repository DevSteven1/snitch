use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command as StdCommand, Stdio};
use std::sync::Arc;
use std::thread;

use snitch_domain::log::{LogLine, LogStore, LogStream};
use snitch_domain::process::{Command, ExitStatus, ProcessId, ProcessSupervisor, SupervisorError};

#[derive(Default)]
pub struct OsProcessSupervisor {
    children: HashMap<ProcessId, Child>,
}

impl OsProcessSupervisor {
    pub fn new() -> Self {
        Self::default()
    }
}

fn stream_lines<R>(reader: R, process_id: ProcessId, stream: LogStream, log_store: Arc<dyn LogStore>)
where
    R: std::io::Read + Send + 'static,
{
    thread::spawn(move || {
        for line in BufReader::new(reader).lines().map_while(Result::ok) {
            log_store.append(LogLine::new(process_id, stream, line));
        }
    });
}

impl ProcessSupervisor for OsProcessSupervisor {
    fn start(
        &mut self,
        command: &Command,
        log_store: Arc<dyn LogStore>,
    ) -> Result<ProcessId, SupervisorError> {
        let mut child = StdCommand::new(command.program())
            .args(command.args())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| SupervisorError::StartFailed(error.to_string()))?;

        let process_id = ProcessId::new(child.id());

        if let Some(stdout) = child.stdout.take() {
            stream_lines(stdout, process_id, LogStream::Stdout, log_store.clone());
        }
        if let Some(stderr) = child.stderr.take() {
            stream_lines(stderr, process_id, LogStream::Stderr, log_store);
        }

        self.children.insert(process_id, child);

        Ok(process_id)
    }

    fn wait(&mut self, process_id: ProcessId) -> Result<ExitStatus, SupervisorError> {
        let child = self
            .children
            .get_mut(&process_id)
            .ok_or(SupervisorError::ProcessNotFound)?;

        let status = child
            .wait()
            .map_err(|error| SupervisorError::WaitFailed(error.to_string()))?;

        self.children.remove(&process_id);

        Ok(ExitStatus::new(status.code()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[derive(Default)]
    struct RecordingLogStore {
        lines: std::sync::Mutex<Vec<LogLine>>,
    }

    impl LogStore for RecordingLogStore {
        fn append(&self, line: LogLine) {
            self.lines.lock().unwrap().push(line);
        }

        fn lines_for(&self, process_id: ProcessId) -> Vec<LogLine> {
            self.lines
                .lock()
                .unwrap()
                .iter()
                .filter(|line| line.process_id() == process_id)
                .cloned()
                .collect()
        }
    }

    #[cfg(windows)]
    fn echoing_command(text: &str) -> Command {
        Command::new("cmd", vec!["/C".to_string(), "echo".to_string(), text.to_string()]).unwrap()
    }

    #[cfg(not(windows))]
    fn echoing_command(text: &str) -> Command {
        Command::new("echo", vec![text.to_string()]).unwrap()
    }

    #[cfg(windows)]
    fn exiting_command() -> Command {
        Command::new("cmd", vec!["/C".to_string(), "exit".to_string(), "0".to_string()]).unwrap()
    }

    #[cfg(not(windows))]
    fn exiting_command() -> Command {
        Command::new("true", vec![]).unwrap()
    }

    #[cfg(windows)]
    fn failing_command() -> Command {
        Command::new("cmd", vec!["/C".to_string(), "exit".to_string(), "1".to_string()]).unwrap()
    }

    #[cfg(not(windows))]
    fn failing_command() -> Command {
        Command::new("false", vec![]).unwrap()
    }

    #[test]
    fn spawns_a_real_process_and_returns_its_id() {
        let mut supervisor = OsProcessSupervisor::new();
        let log_store: Arc<dyn LogStore> = Arc::new(RecordingLogStore::default());

        let process_id = supervisor.start(&exiting_command(), log_store).unwrap();

        assert!(process_id.value() > 0);
    }

    #[test]
    fn fails_to_start_a_command_that_does_not_exist() {
        let mut supervisor = OsProcessSupervisor::new();
        let log_store: Arc<dyn LogStore> = Arc::new(RecordingLogStore::default());
        let command = Command::new("snitch-command-that-does-not-exist", vec![]).unwrap();

        let result = supervisor.start(&command, log_store);

        assert!(result.is_err());
    }

    #[test]
    fn captures_the_process_stdout_into_the_log_store() {
        let mut supervisor = OsProcessSupervisor::new();
        let log_store = Arc::new(RecordingLogStore::default());

        let process_id = supervisor
            .start(&echoing_command("hello-from-snitch"), log_store.clone())
            .unwrap();

        let lines = wait_for_lines(&log_store, process_id);

        assert_eq!(lines[0].stream(), LogStream::Stdout);
        assert_eq!(lines[0].content(), "hello-from-snitch");
    }

    #[test]
    fn waits_for_a_successful_process_and_reports_success() {
        let mut supervisor = OsProcessSupervisor::new();
        let log_store: Arc<dyn LogStore> = Arc::new(RecordingLogStore::default());
        let process_id = supervisor.start(&exiting_command(), log_store).unwrap();

        let status = supervisor.wait(process_id).unwrap();

        assert!(status.success());
    }

    #[test]
    fn waits_for_a_failing_process_and_reports_failure() {
        let mut supervisor = OsProcessSupervisor::new();
        let log_store: Arc<dyn LogStore> = Arc::new(RecordingLogStore::default());
        let process_id = supervisor.start(&failing_command(), log_store).unwrap();

        let status = supervisor.wait(process_id).unwrap();

        assert!(!status.success());
        assert_eq!(status.code(), Some(1));
    }

    #[test]
    fn fails_to_wait_for_an_unknown_process() {
        let mut supervisor = OsProcessSupervisor::new();

        let result = supervisor.wait(ProcessId::new(999_999));

        assert_eq!(result, Err(SupervisorError::ProcessNotFound));
    }

    fn wait_for_lines(store: &Arc<RecordingLogStore>, process_id: ProcessId) -> Vec<LogLine> {
        for _ in 0..50 {
            let lines = store.lines_for(process_id);
            if !lines.is_empty() {
                return lines;
            }
            sleep(Duration::from_millis(20));
        }
        Vec::new()
    }
}
