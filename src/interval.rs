use crate::time_point::TimePoint;
use crate::precision::Precision;

#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub lower: TimePoint,
    pub upper: TimePoint,
}

pub fn to_interval(lower: &TimePoint) -> Interval {
    let year = lower.year;
    let month = lower.month;
    let day = lower.day;

    let upper = match lower.precision {
        Precision::Year => TimePoint {
            year: year + 1,
            month: 1,
            day: 1,
            precision: Precision::Year,
        },

        Precision::Month => {
            if month == 12 {
                TimePoint {
                    year: year + 1,
                    month: 1,
                    day: 1,
                    precision: Precision::Month,
                }
            } else {
                TimePoint {
                    year,
                    month: month + 1,
                    day: 1,
                    precision: Precision::Month,
                }
            }
        }

        Precision::Day => TimePoint {
            year,
            month,
            day: day + 1,
            precision: Precision::Day,
        },
    };

    Interval {
        lower: lower.clone(),
        upper,
    }
}

impl Interval {
    pub fn before(&self, other: &Interval) -> bool {
        self.upper <= other.lower
    }

    pub fn after(&self, other: &Interval) -> bool {
        self.lower >= other.upper
    }
}