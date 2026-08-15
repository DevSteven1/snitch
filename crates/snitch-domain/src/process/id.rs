#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(u32);

impl ProcessId {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_the_underlying_value() {
        let id = ProcessId::new(1234);

        assert_eq!(id.value(), 1234);
    }

    #[test]
    fn two_ids_with_the_same_value_are_equal() {
        assert_eq!(ProcessId::new(42), ProcessId::new(42));
    }

    #[test]
    fn two_ids_with_different_values_are_not_equal() {
        assert_ne!(ProcessId::new(1), ProcessId::new(2));
    }
}
