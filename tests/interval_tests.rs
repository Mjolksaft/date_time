pub use date_time::interval;
pub use date_time::interval::{to_interval, Interval};
pub use date_time::time_point::{time_point};
pub use date_time::precision::Precision;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_precision_becomes_one_year_interval() {
        let t = time_point("2027").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.lower.year, 2027);
        assert_eq!(interval.upper.year, 2028);
        assert_eq!(interval.upper.month, 1);
        assert_eq!(interval.upper.day, 1);
        assert_eq!(interval.upper.precision, Precision::Year);
    }

    #[test]
    fn month_precision_becomes_one_month_interval() {
        let t = time_point("2027-04").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.lower.month, 4);
        assert_eq!(interval.upper.month, 5);
        assert_eq!(interval.upper.day, 1);
        assert_eq!(interval.upper.precision, Precision::Month);
    }

    #[test]
    fn day_precision_becomes_one_day_interval() {
        let t = time_point("2027-04-20").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.lower.day, 20);
        assert_eq!(interval.upper.day, 21);
        assert_eq!(interval.upper.precision, Precision::Day);
    }

    #[test]
    fn hour_precision_becomes_one_hour_interval() {
        let t = time_point("2027-04-20-13").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.lower.hour, 13);
        assert_eq!(interval.upper.hour, 14);
        assert_eq!(interval.upper.precision, Precision::Hour);
    }

    #[test]
    fn minute_precision_becomes_one_minute_interval() {
        let t = time_point("2027-04-20-13-45").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.lower.minute, 45);
        assert_eq!(interval.upper.minute, 46);
        assert_eq!(interval.upper.precision, Precision::Minute);
    }

    #[test]
    fn second_precision_becomes_one_second_interval() {
        let t = time_point("2027-04-20-13-45-30").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.lower.second, 30);
        assert_eq!(interval.upper.second, 31);
        assert_eq!(interval.upper.precision, Precision::Second);
    }

    #[test]
    fn millisecond_precision_becomes_one_millisecond_interval() {
        let t = time_point("2027-04-20-13-45-30-250").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.lower.millisecond, 250);
        assert_eq!(interval.upper.millisecond, 251);
        assert_eq!(interval.upper.precision, Precision::Millisecond);
    }

    #[test]
    fn month_interval_rolls_to_next_year() {
        let t = time_point("2027-12").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.upper.year, 2028);
        assert_eq!(interval.upper.month, 1);
        assert_eq!(interval.upper.day, 1);
    }

    #[test]
    fn day_interval_rolls_to_next_month() {
        let t = time_point("2027-04-30").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.upper.month, 5);
        assert_eq!(interval.upper.day, 1);
    }

    #[test]
    fn hour_interval_rolls_to_next_day() {
        let t = time_point("2027-04-20-23").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.upper.day, 21);
        assert_eq!(interval.upper.hour, 0);
    }

    #[test]
    fn minute_interval_rolls_to_next_hour() {
        let t = time_point("2027-04-20-13-59").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.upper.hour, 14);
        assert_eq!(interval.upper.minute, 0);
    }

    #[test]
    fn second_interval_rolls_to_next_minute() {
        let t = time_point("2027-04-20-13-45-59").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.upper.minute, 46);
        assert_eq!(interval.upper.second, 0);
    }

    #[test]
    fn millisecond_interval_rolls_to_next_second() {
        let t = time_point("2027-04-20-13-45-30-999").unwrap();
        let interval = to_interval(&t).unwrap();

        assert_eq!(interval.upper.second, 31);
        assert_eq!(interval.upper.millisecond, 0);
    }

    #[test]
    fn interval_new_rejects_lower_after_upper() {
        let lower = time_point("2027-04-20").unwrap();
        let upper = time_point("2027-04-19").unwrap();

        let result = Interval::new(lower, upper);

        assert!(result.is_err());
    }
    
}

