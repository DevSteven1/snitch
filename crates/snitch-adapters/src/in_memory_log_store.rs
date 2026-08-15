use std::sync::Mutex;

use snitch_domain::log::{LogLine, LogStore};
use snitch_domain::process::ProcessId;

#[derive(Default)]
pub struct InMemoryLogStore {
    lines: Mutex<Vec<LogLine>>,
}

impl InMemoryLogStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LogStore for InMemoryLogStore {
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

#[cfg(test)]
mod tests {
    use super::*;
    use snitch_domain::log::LogStream;

    #[test]
    fn returns_no_lines_for_a_process_with_none_appended() {
        let store = InMemoryLogStore::new();

        assert!(store.lines_for(ProcessId::new(1)).is_empty());
    }

    #[test]
    fn returns_appended_lines_for_the_matching_process() {
        let store = InMemoryLogStore::new();
        let line = LogLine::new(ProcessId::new(1), LogStream::Stdout, "hello");
        store.append(line.clone());

        assert_eq!(store.lines_for(ProcessId::new(1)), vec![line]);
    }

    #[test]
    fn does_not_return_lines_from_other_processes() {
        let store = InMemoryLogStore::new();
        store.append(LogLine::new(ProcessId::new(1), LogStream::Stdout, "hello"));

        assert!(store.lines_for(ProcessId::new(2)).is_empty());
    }

    #[test]
    fn preserves_append_order() {
        let store = InMemoryLogStore::new();
        store.append(LogLine::new(ProcessId::new(1), LogStream::Stdout, "first"));
        store.append(LogLine::new(ProcessId::new(1), LogStream::Stderr, "second"));

        let lines = store.lines_for(ProcessId::new(1));

        assert_eq!(lines[0].content(), "first");
        assert_eq!(lines[1].content(), "second");
    }
}
