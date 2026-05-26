use crate::precision::Precision;
use crate::time_point::TimePoint;
use crate::truth_values::TruthValue;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interval {
    pub lower: TimePoint,
    pub upper: TimePoint,
}

impl Interval {
    pub fn new(lower: TimePoint, upper: TimePoint) -> Result<Self, String> {
        if lower > upper {
            return Err(String::from(
                "Lower bound must be less than or equal to upper bound",
            ));
        }

        Ok(Self { lower, upper })
    }

    fn relation_to(&self, other: &Interval) -> IntervalRelation {
        if self.upper <= other.lower {
            IntervalRelation::Before
        } else if self.lower >= other.upper {
            IntervalRelation::After
        } else if self.lower == other.lower && self.upper == other.upper {
            IntervalRelation::Equal
        } else if self.lower <= other.lower && other.upper <= self.upper {
            IntervalRelation::Contains
        } else if other.lower <= self.lower && self.upper <= other.upper {
            IntervalRelation::Inside
        } else {
            IntervalRelation::Overlaps
        }
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


pub fn calculate_upper(lower: &TimePoint) -> TimePoint {
    match lower.precision {
        Precision::Year => TimePoint::add_one_year(lower),
        Precision::Month => TimePoint::add_one_month(lower),
        Precision::Day => TimePoint::add_one_day(lower),
        Precision::Hour => TimePoint::add_one_hour(lower),
        Precision::Minute => TimePoint::add_one_minute(lower),
        Precision::Second => TimePoint::add_one_second(lower),
        Precision::Millisecond => TimePoint::add_one_millisecond(lower),
    }
}

pub fn to_interval(lower: &TimePoint) -> Result<Interval, String> {
    let upper = calculate_upper(lower);

    Interval::new(lower.clone(), upper)
}


/////////////////// relations 

#[derive(Debug, PartialEq, Eq)]
enum IntervalRelation {
    Before,
    After,
    Equal,
    Contains,
    Inside,
    Overlaps,
}