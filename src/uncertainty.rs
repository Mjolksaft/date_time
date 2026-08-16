/// A symmetric uncertainty offset `±u` in whole seconds attached to a point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Uncertainty {
    seconds: u64,
}

impl Uncertainty {
    pub fn from_seconds(seconds: u64) -> Self {
        Self { seconds }
    }

    pub fn seconds(self) -> u64 {
        self.seconds
    }
}

impl std::fmt::Display for Uncertainty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "±{}s", self.seconds)
    }
}
