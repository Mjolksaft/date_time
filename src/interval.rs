use crate::time_point::TimePoint;
use crate::precision::Precision;
use crate::truth_values::TruthValue;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interval {
    pub lower: TimePoint,
    pub upper: TimePoint,
    pub lower_key: u64,
    pub upper_key: u64,
}

#[derive(Debug, PartialEq, Eq)]
enum IntervalRelation {
    Before,
    After,
    Equal,
    Contains,
    Inside,
    Overlaps,
}

pub fn interval(lower: &TimePoint, upper: &TimePoint) -> Result<Interval, String> {
    if lower > upper {
        return Err(String::from("Lower bound must be less than or equal to upper bound"));
    }

    Ok(Interval::new(lower.clone(), upper.clone()))
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
    fn relation_to(&self, other: &Interval) -> IntervalRelation {
        if self.upper_key <= other.lower_key {
            IntervalRelation::Before
        } else if self.lower_key >= other.upper_key {
            IntervalRelation::After
        } else if self.lower_key == other.lower_key && self.upper_key == other.upper_key {
            IntervalRelation::Equal
        } else if self.lower_key <= other.lower_key && other.upper_key <= self.upper_key {
            IntervalRelation::Contains
        } else if other.lower_key <= self.lower_key && self.upper_key <= other.upper_key {
            IntervalRelation::Inside
        } else {
            IntervalRelation::Overlaps
        }
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

    pub fn before(&self, other: &Interval) -> TruthValue {
        match self.relation_to(other) {
            IntervalRelation::Before => TruthValue::True,
            _ => TruthValue::False,
        }
    }

    pub fn after(&self, other: &Interval) -> TruthValue {
        match self.relation_to(other) {
            IntervalRelation::After => TruthValue::True,
            _ => TruthValue::False,
        }
    }

    pub fn equals(&self, other: &Interval) -> TruthValue {
        match self.relation_to(other) {
            IntervalRelation::Equal => TruthValue::True,
            _ => TruthValue::False,
        }
    }

    pub fn contains(&self, other: &Interval) -> TruthValue {
        match self.relation_to(other) {
            IntervalRelation::Contains | IntervalRelation::Equal => TruthValue::True,
            _ => TruthValue::False,
        }
    }

    pub fn overlaps(&self, other: &Interval) -> TruthValue {
        match self.relation_to(other) {
            IntervalRelation::Before | IntervalRelation::After => TruthValue::False,
            _ => TruthValue::True,
        }
    }
}
