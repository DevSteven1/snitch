#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogStream {
    Stdout,
    Stderr,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stdout_and_stderr_are_not_equal() {
        assert_ne!(LogStream::Stdout, LogStream::Stderr);
    }

    #[test]
    fn a_stream_equals_itself() {
        assert_eq!(LogStream::Stdout, LogStream::Stdout);
    }
}
