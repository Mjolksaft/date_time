use crate::precision::Precision;
use crate::time_point::TimePoint;
use crate::truth_values::TruthValue;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interval {
    pub lower: TimePoint,
    pub upper: TimePoint,
    pub lower_key: u64,
    pub upper_key: u64,
}

pub fn interval(lower: &TimePoint, upper: &TimePoint) -> Result<Interval, String> {
    if lower > upper {
        return Err(String::from(
            "Lower bound must be less than or equal to upper bound",
        ));
    }

    Ok(Interval::new(
        clear_uncertainty(lower.clone()),
        clear_uncertainty(upper.clone()),
    ))
}

pub fn calculate_upper(lower: &TimePoint) -> TimePoint {
    match &lower.precision {
        Precision::Year => TimePoint::add_one_year(lower),
        Precision::Month => TimePoint::add_one_month(lower),
        Precision::Day => TimePoint::add_one_day(lower),
        Precision::Hour => TimePoint::add_one_hour(lower),
        Precision::Minute => TimePoint::add_one_minute(lower),
        Precision::Second => TimePoint::add_one_second(lower),
        Precision::Millisecond => TimePoint::add_one_millisecond(lower),
    }
}

fn clear_uncertainty(point: TimePoint) -> TimePoint {
    TimePoint {
        uncertainty: None,
        ..point
    }
}

pub fn to_interval(point: &TimePoint, upper: Option<&TimePoint>) -> Result<Interval, String> {
    let uncertainty = point.uncertainty;

    match upper {
        Some(upper_point) => {
            if point > upper_point {
                return Err(String::from(
                    "Lower bound must be less than or equal to upper bound",
                ));
            }
            Ok(Interval::new(
                clear_uncertainty(point.clone()),
                clear_uncertainty(upper_point.clone()),
            ))
        }
        None => match uncertainty {
            Some(u) => Ok(Interval::new(
                clear_uncertainty(point.sub_seconds(u.seconds())),
                clear_uncertainty(point.add_seconds(u.seconds())),
            )),
            None => Ok(Interval::new(point.clone(), calculate_upper(point))),
        },
    }
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

    fn certainly_disjoint(&self, other: &Interval) -> bool {
        self.upper_key <= other.lower_key || other.upper_key <= self.lower_key
    }

    fn certainly_overlapping(&self, other: &Interval) -> bool {
        self.lower_key < other.upper_key && other.lower_key < self.upper_key
    }

    pub fn before(&self, other: &Interval) -> TruthValue {
        if self.upper_key <= other.lower_key {
            TruthValue::True
        } else if self.lower_key >= other.upper_key {
            TruthValue::False
        } else {
            TruthValue::Unknown
        }
    }

    pub fn after(&self, other: &Interval) -> TruthValue {
        if self.lower_key >= other.upper_key {
            TruthValue::True
        } else if self.upper_key <= other.lower_key {
            TruthValue::False
        } else {
            TruthValue::Unknown
        }
    }

    pub fn equals(&self, other: &Interval) -> TruthValue {
        if self.lower_key == other.lower_key && self.upper_key == other.upper_key {
            TruthValue::True
        } else if self.certainly_disjoint(other) {
            TruthValue::False
        } else {
            TruthValue::Unknown
        }
    }

    pub fn contains(&self, other: &Interval) -> TruthValue {
        if self.lower_key <= other.lower_key && other.upper_key <= self.upper_key {
            TruthValue::True
        } else if self.certainly_disjoint(other) {
            TruthValue::False
        } else {
            TruthValue::Unknown
        }
    }

    pub fn overlaps(&self, other: &Interval) -> TruthValue {
        if self.certainly_overlapping(other) {
            TruthValue::True
        } else if self.certainly_disjoint(other) {
            TruthValue::False
        } else {
            TruthValue::Unknown
        }
    }
}
