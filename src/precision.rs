#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Precision {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}