use crate::process::ProcessId;

use super::LogLine;

pub trait LogStore: Send + Sync {
    fn append(&self, line: LogLine);
    fn lines_for(&self, process_id: ProcessId) -> Vec<LogLine>;
}
