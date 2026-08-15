use crate::process::ProcessId;

use super::LogStream;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogLine {
    process_id: ProcessId,
    stream: LogStream,
    content: String,
}

impl LogLine {
    pub fn new(process_id: ProcessId, stream: LogStream, content: impl Into<String>) -> Self {
        Self {
            process_id,
            stream,
            content: content.into(),
        }
    }

    pub fn process_id(&self) -> ProcessId {
        self.process_id
    }

    pub fn stream(&self) -> LogStream {
        self.stream
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_its_fields() {
        let line = LogLine::new(ProcessId::new(1), LogStream::Stdout, "hello");

        assert_eq!(line.process_id(), ProcessId::new(1));
        assert_eq!(line.stream(), LogStream::Stdout);
        assert_eq!(line.content(), "hello");
    }
}
