/// Result of an interval comparison when partial information is present:
/// certain, certainly-not, or unknown (the spans could still overlap).
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
