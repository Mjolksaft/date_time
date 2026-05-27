use date_time::time_point::time_point;
use date_time::interval::to_interval;
use date_time::truth_values::TruthValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meets_returns_true_when_first_ends_where_second_starts() {
        let a = to_interval(&time_point("2027-04-20").unwrap()).unwrap();
        let b = to_interval(&time_point("2027-04-21").unwrap()).unwrap();

        assert_eq!(a.meets(&b), TruthValue::True);
    }

    #[test]
    fn met_by_returns_true_when_first_starts_where_second_ends() {
        let a = to_interval(&time_point("2027-04-21").unwrap()).unwrap();
        let b = to_interval(&time_point("2027-04-20").unwrap()).unwrap();

        assert_eq!(a.met_by(&b), TruthValue::True);
    }

    #[test]
    fn starts_returns_true_when_same_start_but_first_ends_earlier() {
        let day = to_interval(&time_point("2027-04-01").unwrap()).unwrap();
        let month = to_interval(&time_point("2027-04").unwrap()).unwrap();

        assert_eq!(day.starts(&month), TruthValue::True);
    }

    #[test]
    fn started_by_returns_true_when_same_start_but_first_ends_later() {
        let month = to_interval(&time_point("2027-04").unwrap()).unwrap();
        let day = to_interval(&time_point("2027-04-01").unwrap()).unwrap();

        assert_eq!(month.started_by(&day), TruthValue::True);
    }

    #[test]
    fn finishes_returns_true_when_same_end_but_first_starts_later() {
        let day = to_interval(&time_point("2027-04-30").unwrap()).unwrap();
        let month = to_interval(&time_point("2027-04").unwrap()).unwrap();

        assert_eq!(day.finishes(&month), TruthValue::True);
    }

    #[test]
    fn finished_by_returns_true_when_same_end_but_first_starts_earlier() {
        let month = to_interval(&time_point("2027-04").unwrap()).unwrap();
        let day = to_interval(&time_point("2027-04-30").unwrap()).unwrap();

        assert_eq!(month.finished_by(&day), TruthValue::True);
    }

    #[test]
    fn during_returns_true_when_first_is_strictly_inside_second() {
        let day = to_interval(&time_point("2027-04-20").unwrap()).unwrap();
        let month = to_interval(&time_point("2027-04").unwrap()).unwrap();

        assert_eq!(day.during(&month), TruthValue::True);
    }
}