pub use date_time::time_point::{time_point, TimePoint};
pub use date_time::precision::Precision;
pub use date_time::time_zone::TimeZone;
pub use date_time::truth_values::TruthValue;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timepoint_before_returns_true() {
        let a = time_point("2027-04-20").unwrap();
        let b = time_point("2027-04-21").unwrap();

        assert_eq!(a.before(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn timepoint_after_returns_true() {
        let a = time_point("2027-04-21").unwrap();
        let b = time_point("2027-04-20").unwrap();

        assert_eq!(a.after(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn timepoint_equals_returns_true_for_same_precision() {
        let a = time_point("2027-04-20").unwrap();
        let b = time_point("2027-04-20").unwrap();

        assert_eq!(a.equals(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn year_contains_month() {
        let year = time_point("2027").unwrap();
        let month = time_point("2027-04").unwrap();

        assert_eq!(year.contains(&month).unwrap(), TruthValue::True);
    }

    #[test]
    fn month_overlaps_day_inside_month() {
        let month = time_point("2027-04").unwrap();
        let day = time_point("2027-04-20").unwrap();

        assert_eq!(month.overlaps(&day).unwrap(), TruthValue::True);
    }

    #[test]
    fn adjacent_days_do_not_overlap() {
        let a = time_point("2027-04-20").unwrap();
        let b = time_point("2027-04-21").unwrap();

        assert_eq!(a.overlaps(&b).unwrap(), TruthValue::False);
    }
}

