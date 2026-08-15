#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExitStatus {
    code: Option<i32>,
}

impl ExitStatus {
    pub fn new(code: Option<i32>) -> Self {
        Self { code }
    }

    pub fn code(&self) -> Option<i32> {
        self.code
    }

    pub fn success(&self) -> bool {
        self.code == Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_zero_code_is_a_success() {
        let status = ExitStatus::new(Some(0));

        assert!(status.success());
    }

    #[test]
    fn a_non_zero_code_is_not_a_success() {
        let status = ExitStatus::new(Some(1));

        assert!(!status.success());
    }

    #[test]
    fn no_code_is_not_a_success() {
        let status = ExitStatus::new(None);

        assert!(!status.success());
    }

    #[test]
    fn exposes_the_underlying_code() {
        let status = ExitStatus::new(Some(42));

        assert_eq!(status.code(), Some(42));
    }
}
