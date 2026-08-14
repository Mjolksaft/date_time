#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Precision {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}
