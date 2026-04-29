use crate::time_point::TimePoint;
use crate::precision::Precision;
use crate::truth_values::TruthValue;
use crate::util::{days_in_month};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interval {
    pub lower: TimePoint,
    pub upper: TimePoint,
    pub lower_key: u64,
    pub upper_key: u64,
}

pub fn interval(lower: &TimePoint, upper: &TimePoint) -> Result<Interval, String> {
    if lower > upper {
        return Err(String::from("Lower bound must be less than or equal to upper bound"));
    }

    Ok(Interval::new(lower.clone(), upper.clone()))
}

pub fn calculate_upper(lower: &TimePoint) -> TimePoint {
    match lower.precision {
        Precision::Year => add_one_year(lower),
        Precision::Month => add_one_month(lower),
        Precision::Day => add_one_day(lower),
        Precision::Hour => add_one_hour(lower),
        Precision::Minute => add_one_minute(lower),
        Precision::Second => add_one_second(lower),
    }
}

fn start_of(
    year: u32,
    month: u32,
    day: u32,
    precision: Precision,
) -> TimePoint {
    TimePoint {
        year,
        month,
        day,
        hour: 0,
        minute: 0,
        second: 0,
        precision,
    }
}
fn add_one_second(t: &TimePoint) -> TimePoint {
    if t.second == 59 {
        let next_minute = add_one_minute(t);

        TimePoint {
            precision: t.precision.clone(),
            ..next_minute
        }
    } else {
        TimePoint {
            year: t.year,
            month: t.month,
            day: t.day,
            hour: t.hour,
            minute: t.minute,
            second: t.second + 1,
            precision: t.precision.clone(),
        }
    }
}

fn add_one_minute(t: &TimePoint) -> TimePoint {
    if t.minute == 59 {
        let next_hour = add_one_hour(t);

        TimePoint {
            precision: t.precision.clone(),
            ..next_hour
        }
    } else {
        TimePoint {
            year: t.year,
            month: t.month,
            day: t.day,
            hour: t.hour,
            minute: t.minute + 1,
            second: 0,
            precision: t.precision.clone(),
        }
    }
}

fn add_one_hour(t: &TimePoint) -> TimePoint {
    if t.hour == 23 {
        let next_day = add_one_day(t);

        TimePoint {
            precision: t.precision.clone(),
            ..next_day
        }
    } else {
        TimePoint {
            year: t.year,
            month: t.month,
            day: t.day,
            hour: t.hour + 1,
            minute: 0,
            second: 0,
            precision: t.precision.clone(),
        }
    }
}

fn add_one_day(t: &TimePoint) -> TimePoint {
    let days = days_in_month(t.year, t.month);

    if t.day == days {
        if t.month == 12 {
            start_of(t.year + 1, 1, 1, t.precision.clone())
        } else {
            start_of(t.year, t.month + 1, 1, t.precision.clone())
        }
    } else {
        start_of(t.year, t.month, t.day + 1, t.precision.clone())
    }
}

fn add_one_month(t: &TimePoint) -> TimePoint {
    if t.month == 12 {
        start_of(t.year + 1, 1, 1, t.precision.clone())
    } else {
        start_of(t.year, t.month + 1, 1, t.precision.clone())
    }
}

fn add_one_year(t: &TimePoint) -> TimePoint {
    start_of(t.year + 1, 1, 1, t.precision.clone())
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

    Ok(Interval::new(lower.clone(), upper))
}

impl Interval {
    pub fn new(lower: TimePoint, upper: TimePoint) -> Self {
        let lower_key = lower.boundary_key();
        let upper_key = upper.boundary_key();

        Self {
            lower,
            upper,
            lower_key,
            upper_key,
        }
    }
    
    pub fn lower_key(&self) -> u64 {
        self.lower.boundary_key()
    }

    pub fn upper_key(&self) -> u64 {
        self.upper.boundary_key()
    }
        
    pub fn before(&self, other: &Interval) -> TruthValue {
        if self.upper_key <= other.lower_key {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }

    pub fn after(&self, other: &Interval) -> TruthValue {
        if self.lower_key >= other.upper_key {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }

    pub fn contains(&self, other: &Interval) -> TruthValue {
        if self.lower_key <= other.lower_key && other.upper_key <= self.upper_key {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }

    pub fn overlaps(&self, other: &Interval) -> TruthValue {
        if self.lower_key < other.upper_key && other.lower_key < self.upper_key {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }

    pub fn equals(&self, other: &Interval) -> TruthValue {
        if self.lower_key == other.lower_key && self.upper_key == other.upper_key {
            TruthValue::True
        } else {
            TruthValue::False
        }
    }
}

