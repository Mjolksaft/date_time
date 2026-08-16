use crate::precision::Precision;
use crate::time_point::TimePoint;
use crate::truth_values::TruthValue;

/// A half-open span `[lower, upper)` with precomputed ordered boundary keys.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Interval {
    pub lower: TimePoint,
    pub upper: TimePoint,
    pub lower_key: u64,
    pub upper_key: u64,
}

/// The thirteen possible relations between two intervals (Allen's interval algebra).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllenRelation {
    Before,
    After,
    Meets,
    MetBy,
    Overlaps,
    OverlappedBy,
    Contains,
    During,
    Starts,
    StartedBy,
    Finishes,
    FinishedBy,
    Equal,
}

impl std::fmt::Display for AllenRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            AllenRelation::Before => "before",
            AllenRelation::After => "after",
            AllenRelation::Meets => "meets",
            AllenRelation::MetBy => "met-by",
            AllenRelation::Overlaps => "overlaps",
            AllenRelation::OverlappedBy => "overlapped-by",
            AllenRelation::Contains => "contains",
            AllenRelation::During => "during",
            AllenRelation::Starts => "starts",
            AllenRelation::StartedBy => "started-by",
            AllenRelation::Finishes => "finishes",
            AllenRelation::FinishedBy => "finished-by",
            AllenRelation::Equal => "equal",
        };

        f.write_str(name)
    }
}

/// Builds `[lower, upper)` from explicit bounds; strips any uncertainty.
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

/// Computes the upper bound that the given precision's point denotes, i.e. the
/// boundary of the unit the point resolves to (e.g. next day for a Day point).
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

/// Turns a point into an interval: with an explicit `upper` it is `[point, upper)`,
/// with an uncertainty `t ± u` it becomes `[t-u, t+u)`, and otherwise it spans
/// the unit implied by its precision (uncertainty takes precedence over precision).
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

    // Three-valued logic helpers on the ordered keys: `certainly_disjoint`
    // means the relation is decided, otherwise the answer may be Unknown.
    fn certainly_disjoint(&self, other: &Interval) -> bool {
        self.upper_key <= other.lower_key || other.upper_key <= self.lower_key
    }

    fn certainly_overlapping(&self, other: &Interval) -> bool {
        self.lower_key < other.upper_key && other.lower_key < self.upper_key
    }

    /// Three-valued `before`: True when certainly before, False when certainly
    /// after, Unknown when the spans could still overlap.
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

    /// Exact Allen relation when the boundaries are fully ordered; errors if
    /// either interval is degenerate (lower == upper).
    pub fn allen_relation(&self, other: &Interval) -> Result<AllenRelation, String> {
        self.validate_interval()?;
        other.validate_interval()?;

        let a_start = self.lower_key;
        let a_end = self.upper_key;
        let b_start = other.lower_key;
        let b_end = other.upper_key;

        if a_end < b_start {
            return Ok(AllenRelation::Before);
        }
        if a_end == b_start {
            return Ok(AllenRelation::Meets);
        }

        if a_start < b_start && b_end < a_end {
            return Ok(AllenRelation::Contains);
        }

        if a_start < b_start && b_start < a_end && a_end < b_end {
            return Ok(AllenRelation::Overlaps);
        }

        if a_start < b_start && a_end == b_end {
            return Ok(AllenRelation::FinishedBy);
        }

        if a_start == b_start && a_end < b_end {
            return Ok(AllenRelation::Starts);
        }

        if a_start == b_start && a_end == b_end {
            return Ok(AllenRelation::Equal);
        }

        if b_start < a_start && a_end < b_end {
            return Ok(AllenRelation::During);
        }

        if a_start == b_start && a_end > b_end {
            return Ok(AllenRelation::StartedBy);
        }

        if b_start < a_start && a_end == b_end {
            return Ok(AllenRelation::Finishes);
        }

        if b_start < a_start && a_start < b_end && b_end < a_end {
            return Ok(AllenRelation::OverlappedBy);
        }

        if b_end == a_start {
            return Ok(AllenRelation::MetBy);
        }

        if b_end < a_start {
            return Ok(AllenRelation::After);
        }

        Err(String::from("Unable to classify interval relation"))
    }

    fn validate_interval(&self) -> Result<(), String> {
        if self.lower_key >= self.upper_key {
            return Err(String::from(
                "Interval is invalid, lower bound must be less than upper bound",
            ));
        }

        Ok(())
    }
}
