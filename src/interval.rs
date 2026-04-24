use crate::time_point::TimePoint;
use crate::precision::Precision;
use crate::truth_values::TruthValue;
use crate::util::{days_in_month};

#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub lower: TimePoint,
    pub upper: TimePoint,
}

pub fn interval(lower: &TimePoint, upper: &TimePoint) -> Result<Interval, String> {
    if lower > upper {
        return Err(String::from("Lower bound must be less than or equal to upper bound"));
    }

    Ok(Interval { lower: lower.clone(), upper: upper.clone()})
}

pub fn calculate_upper(lower: &TimePoint) -> TimePoint {
    let year = lower.year;
    let month = lower.month;
    let day = lower.day;

    match lower.precision {
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

        Precision::Day => {
            let days_in_current_month = days_in_month(year, month);
            if day == days_in_current_month {
                if month == 12 {
                    TimePoint {
                        year: year + 1,
                        month: 1,
                        day: 1,
                        precision: Precision::Day,
                    }
                } else {
                    TimePoint {
                        year,
                        month: month + 1,
                        day: 1,
                        precision: Precision::Day,
                    }
                }
            } else {
                TimePoint {
                year,
                month,
                day: day + 1,
                precision: Precision::Day,
                }
            }
        },
    }
}

pub fn to_interval(lower: &TimePoint, upper: Option<&TimePoint>) -> Result<Interval, String> {
    let upper: TimePoint = match upper {
        Some(upper_point) => {
            if lower > upper_point {
                return Err(String::from("Lower bound must be less than or equal to upper bound"));
            }
            upper_point.clone()
        }
        None => calculate_upper(lower),
    };

    Ok(Interval {
        lower: lower.clone(),
        upper,
    })
}

impl Interval {
    pub fn before(&self, other: &Interval) -> TruthValue {
        if self.upper <= other.lower {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }

    pub fn after(&self, other: &Interval) -> TruthValue {
        if self.lower >= other.upper {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }

pub fn equals(&self, other: &Interval) -> TruthValue {
    if self.lower == other.lower && self.upper == other.upper {
        TruthValue::True
    } else {
        TruthValue::False
    }
}

    pub fn overlaps(&self, other: &Interval) -> TruthValue {
        if self.lower < other.upper && self.upper > other.lower {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }

    pub fn contains(&self, other: &Interval) -> TruthValue {
        if self.lower <= other.lower && other.upper <= self.upper {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }
}

