use crate::precision::Precision;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimePoint {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub precision: Precision,
}

impl Ord for TimePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month, self.day)
            .cmp(&(other.year, other.month, other.day))
    }
}

impl PartialOrd for TimePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TimePoint {
    pub fn equals(&self, other: &TimePoint) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.precision == other.precision // maybe dont check precision?
    }

    pub fn before(&self, other: &TimePoint) -> bool {
        let a = crate::interval::to_interval(self);
        let b = crate::interval::to_interval(other);
        a.before(&b)
    }

    pub fn after(&self, other: &TimePoint) -> bool {
        let a = crate::interval::to_interval(self);
        let b = crate::interval::to_interval(other);
        a.after(&b)
    }
}
