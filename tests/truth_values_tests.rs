pub use date_time::truth_values::TruthValue;
pub use date_time::interval;
pub use date_time::interval::{to_interval};
pub use date_time::time_point::{time_point};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn before_returns_true_when_interval_ends_before_other_starts() {
        let a = to_interval(&time_point("2027-04-20").unwrap()).unwrap();
        let b = to_interval(&time_point("2027-04-21").unwrap()).unwrap();

        assert_eq!(a.before(&b), TruthValue::True);
    }

    #[test]
    fn after_returns_true_when_interval_starts_after_other_ends() {
        let a = to_interval(&time_point("2027-04-21").unwrap()).unwrap();
        let b = to_interval(&time_point("2027-04-20").unwrap()).unwrap();

        assert_eq!(a.after(&b), TruthValue::True);
    }

    #[test]
    fn equals_returns_true_for_same_interval() {
        let a = to_interval(&time_point("2027-04-20").unwrap()).unwrap();
        let b = to_interval(&time_point("2027-04-20").unwrap()).unwrap();

        assert_eq!(a.equals(&b), TruthValue::True);
    }

    #[test]
    fn contains_returns_true_for_year_containing_month() {
        let year = to_interval(&time_point("2027").unwrap()).unwrap();
        let month = to_interval(&time_point("2027-04").unwrap()).unwrap();

        assert_eq!(year.contains(&month), TruthValue::True);
    }

    #[test]
    fn overlaps_returns_true_when_intervals_intersect() {
        let month = to_interval(&time_point("2027-04").unwrap()).unwrap();
        let day = to_interval(&time_point("2027-04-20").unwrap()).unwrap();

        assert_eq!(month.overlaps(&day), TruthValue::True);
    }

    #[test]
    fn overlaps_returns_false_for_adjacent_intervals() {
        let a = to_interval(&time_point("2027-04-20").unwrap()).unwrap();
        let b = to_interval(&time_point("2027-04-21").unwrap()).unwrap();

        assert_eq!(a.overlaps(&b), TruthValue::False);
    }

    #[test]
    fn day_does_not_contain_month() {
        let day = to_interval(&time_point("2027-04-20").unwrap()).unwrap();
        let month = to_interval(&time_point("2027-04").unwrap()).unwrap();

        assert_eq!(day.contains(&month), TruthValue::False);
    }

    #[test]
    fn month_is_before_next_month() {
        let april = to_interval(&time_point("2027-04").unwrap()).unwrap();
        let may = to_interval(&time_point("2027-05").unwrap()).unwrap();

        assert_eq!(april.before(&may), TruthValue::True);
    }

    
}
