use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessName(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProcessNameError {
    #[error("process name must not be empty")]
    Empty,
    #[error("process name must not contain whitespace")]
    ContainsWhitespace,
}

impl ProcessName {
    pub fn new(value: impl Into<String>) -> Result<Self, ProcessNameError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ProcessNameError::Empty);
        }
        if value.chars().any(char::is_whitespace) {
            return Err(ProcessNameError::ContainsWhitespace);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProcessName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_non_empty_single_word_name() {
        let name = ProcessName::new("api-gateway").unwrap();

        assert_eq!(name.as_str(), "api-gateway");
    }

    #[test]
    fn rejects_an_empty_name() {
        let result = ProcessName::new("");

        assert_eq!(result, Err(ProcessNameError::Empty));
    }

    #[test]
    fn rejects_a_blank_name() {
        let result = ProcessName::new("   ");

        assert_eq!(result, Err(ProcessNameError::Empty));
    }

    #[test]
    fn rejects_a_name_containing_whitespace() {
        let result = ProcessName::new("api gateway");

        assert_eq!(result, Err(ProcessNameError::ContainsWhitespace));
    }
}
