use std::io::{BufRead, BufReader};
use std::process::{Command as StdCommand, Stdio};
use std::sync::Arc;
use std::thread;

use snitch_domain::log::{LogLine, LogStore, LogStream};
use snitch_domain::process::{Command, ProcessId, ProcessSupervisor, SupervisorError};

#[derive(Default)]
pub struct OsProcessSupervisor;

impl OsProcessSupervisor {
    pub fn new() -> Self {
        Self
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

        Ok(process_id)
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
