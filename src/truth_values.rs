#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TruthValue {
    True,
    False,
    Unknown,
}

impl From<bool> for TruthValue {
    fn from(value: bool) -> Self {
        if value {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }
}