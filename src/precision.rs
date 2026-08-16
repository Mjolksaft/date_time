/// The granularity a `TimePoint` resolves to; it determines the interval size
/// when the point is expanded (e.g. a `Day` point spans one whole day).
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
