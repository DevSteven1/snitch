#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Command {
    program: String,
    args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CommandError {
    #[error("command program must not be empty")]
    EmptyProgram,
}

impl Command {
    pub fn new(program: impl Into<String>, args: Vec<String>) -> Result<Self, CommandError> {
        let program = program.into();
        if program.trim().is_empty() {
            return Err(CommandError::EmptyProgram);
        }
        Ok(Self { program, args })
    }

    pub fn program(&self) -> &str {
        &self.program
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_command_with_program_and_args() {
        let command = Command::new("cargo", vec!["run".to_string()]).unwrap();

        assert_eq!(command.program(), "cargo");
        assert_eq!(command.args(), ["run".to_string()]);
    }

    #[test]
    fn builds_a_command_with_no_args() {
        let command = Command::new("ls", vec![]).unwrap();

        assert!(command.args().is_empty());
    }

    #[test]
    fn rejects_an_empty_program() {
        let result = Command::new("", vec![]);

        assert_eq!(result, Err(CommandError::EmptyProgram));
    }

    #[test]
    fn rejects_a_blank_program() {
        let result = Command::new("   ", vec![]);

        assert_eq!(result, Err(CommandError::EmptyProgram));
    }
}
