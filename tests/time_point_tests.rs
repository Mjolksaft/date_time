pub use date_time::time_point::{time_point, TimePoint};
pub use date_time::precision::Precision;
pub use date_time::time_zone::TimeZone;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_year_precision_with_defaults() {
        let result = TimePoint::new(2027, None, None, None, None, None, None).unwrap();

        assert_eq!(result.year, 2027);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);
        assert_eq!(result.hour, 0);
        assert_eq!(result.minute, 0);
        assert_eq!(result.second, 0);
        assert_eq!(result.millisecond, 0);
        assert_eq!(result.precision, Precision::Year);
        assert_eq!(result.zone, TimeZone::UTC);
    }

    #[test]
    fn constructs_millisecond_precision() {
        let result = TimePoint::new(
            2027,
            Some(4),
            Some(20),
            Some(13),
            Some(45),
            Some(30),
            Some(250),
        )
        .unwrap();

        assert_eq!(result.year, 2027);
        assert_eq!(result.month, 4);
        assert_eq!(result.day, 20);
        assert_eq!(result.hour, 13);
        assert_eq!(result.minute, 45);
        assert_eq!(result.second, 30);
        assert_eq!(result.millisecond, 250);
        assert_eq!(result.precision, Precision::Millisecond);
    }

    #[test]
    fn parses_second_precision() {
        let result = time_point("2027-04-20-13-45-30").unwrap();

        assert_eq!(result.year, 2027);
        assert_eq!(result.month, 4);
        assert_eq!(result.day, 20);
        assert_eq!(result.hour, 13);
        assert_eq!(result.minute, 45);
        assert_eq!(result.second, 30);
        assert_eq!(result.millisecond, 0);
        assert_eq!(result.precision, Precision::Second);
    }

    #[test]
    fn parses_millisecond_precision() {
        let result = time_point("2027-04-20-13-45-30-250").unwrap();

        assert_eq!(result.second, 30);
        assert_eq!(result.millisecond, 250);
        assert_eq!(result.precision, Precision::Millisecond);
    }

    #[test]
    fn rejects_invalid_month() {
        assert!(time_point("2027-13").is_err());
    }

    #[test]
    fn rejects_invalid_day() {
        assert!(time_point("2027-04-40").is_err());
    }

    #[test]
    fn rejects_invalid_hour() {
        assert!(time_point("2027-04-20-24").is_err());
    }

    #[test]
    fn rejects_invalid_minute() {
        assert!(time_point("2027-04-20-13-60").is_err());
    }

    #[test]
    fn rejects_invalid_millisecond() {
        assert!(time_point("2027-04-20-13-45-30-1000").is_err());
    }

    #[test]
    fn rejects_too_many_parts() {
        assert!(time_point("2027-04-20-13-45-30-250-1").is_err());
    }

    #[test]
    fn add_one_millisecond_increments_normally() {
        let t = time_point("2027-04-20-13-45-30-250").unwrap();
        let result = TimePoint::add_one_millisecond(&t);

        assert_eq!(result.second, 30);
        assert_eq!(result.millisecond, 251);
    }

    #[test]
    fn millisecond_rolls_to_next_second() {
        let t = time_point("2027-04-20-13-45-30-999").unwrap();
        let result = TimePoint::add_one_millisecond(&t);

        assert_eq!(result.second, 31);
        assert_eq!(result.millisecond, 0);
    }

    #[test]
    fn second_rolls_to_next_minute() {
        let t = time_point("2027-04-20-13-45-59").unwrap();
        let result = TimePoint::add_one_second(&t);

        assert_eq!(result.minute, 46);
        assert_eq!(result.second, 0);
        assert_eq!(result.millisecond, 0);
    }

    #[test]
    fn minute_rolls_to_next_hour() {
        let t = time_point("2027-04-20-13-59-59").unwrap();
        let result = TimePoint::add_one_second(&t);

        assert_eq!(result.hour, 14);
        assert_eq!(result.minute, 0);
        assert_eq!(result.second, 0);
    }

    #[test]
    fn hour_rolls_to_next_day() {
        let t = time_point("2027-04-20-23-59-59").unwrap();
        let result = TimePoint::add_one_second(&t);

        assert_eq!(result.day, 21);
        assert_eq!(result.hour, 0);
        assert_eq!(result.minute, 0);
        assert_eq!(result.second, 0);
    }

    #[test]
    fn day_rolls_to_next_month() {
        let t = time_point("2027-04-30-23-59-59").unwrap();
        let result = TimePoint::add_one_second(&t);

        assert_eq!(result.month, 5);
        assert_eq!(result.day, 1);
    }

    #[test]
    fn month_rolls_to_next_year() {
        let t = time_point("2027-12-31-23-59-59").unwrap();
        let result = TimePoint::add_one_second(&t);

        assert_eq!(result.year, 2028);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);
    }

    #[test]
    fn supports_leap_year_date() {
        assert!(time_point("2028-02-29").is_ok());
    }

    #[test]
    fn rejects_non_leap_year_february_29() {
        assert!(time_point("2027-02-29").is_err());
    }

    #[test]
    fn now_utc_returns_valid_millisecond_precision_timepoint() {
        let now = TimePoint::now_utc();

        assert_eq!(now.precision, Precision::Millisecond);
        assert!(now.month >= 1 && now.month <= 12);
        assert!(now.day >= 1 && now.day <= 31);
        assert!(now.hour <= 23);
        assert!(now.minute <= 59);
        assert!(now.second <= 59);
        assert!(now.millisecond <= 999);
    }

    #[test]
    fn sub_one_millisecond_decrements_normally() {
        let t = time_point("2027-04-20-13-45-30-250").unwrap();
        let result = TimePoint::sub_one_millisecond(&t);

        assert_eq!(result.second, 30);
        assert_eq!(result.millisecond, 249);
    }

    #[test]
    fn sub_one_millisecond_rolls_to_previous_second() {
        let t = time_point("2027-04-20-13-45-30-000").unwrap();
        let result = TimePoint::sub_one_millisecond(&t);

        assert_eq!(result.second, 29);
        assert_eq!(result.millisecond, 999);
    }

    #[test]
    fn sub_one_second_decrements_normally() {
        let t = time_point("2027-04-20-13-45-30").unwrap();
        let result = TimePoint::sub_one_second(&t);

        assert_eq!(result.second, 29);
        assert_eq!(result.millisecond, 0);
    }

    #[test]
    fn sub_one_second_rolls_to_previous_minute() {
        let t = time_point("2027-04-20-13-45-00").unwrap();
        let result = TimePoint::sub_one_second(&t);

        assert_eq!(result.minute, 44);
        assert_eq!(result.second, 59);
        assert_eq!(result.millisecond, 0);
    }

    #[test]
    fn sub_one_minute_rolls_to_previous_hour() {
        let t = time_point("2027-04-20-13-00-00").unwrap();
        let result = TimePoint::sub_one_minute(&t);

        assert_eq!(result.hour, 12);
        assert_eq!(result.minute, 59);
        assert_eq!(result.second, 59);
    }

    #[test]
    fn sub_one_hour_rolls_to_previous_day() {
        let t = time_point("2027-04-20-00-00-00").unwrap();
        let result = TimePoint::sub_one_hour(&t);

        assert_eq!(result.day, 19);
        assert_eq!(result.hour, 23);
        assert_eq!(result.minute, 59);
        assert_eq!(result.second, 59);
    }

    #[test]
    fn sub_one_day_rolls_to_previous_month() {
        let t = time_point("2027-04-01").unwrap();
        let result = TimePoint::sub_one_day(&t);

        assert_eq!(result.month, 3);
        assert_eq!(result.day, 31);
    }

    #[test]
    fn sub_one_day_rolls_to_previous_year() {
        let t = time_point("2027-01-01").unwrap();
        let result = TimePoint::sub_one_day(&t);

        assert_eq!(result.year, 2026);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 31);
    }

    #[test]
    fn sub_multiple_milliseconds() {
        let t = time_point("2027-04-20-13-45-30-250").unwrap();
        let result = t.sub_milliseconds(251);

        assert_eq!(result.second, 29);
        assert_eq!(result.millisecond, 999);
    }

    #[test]
    fn sub_multiple_seconds() {
        let t = time_point("2027-04-20-13-45-30").unwrap();
        let result = t.sub_seconds(31);

        assert_eq!(result.minute, 44);
        assert_eq!(result.second, 59);
    }

    #[test]
    fn sub_multiple_minutes() {
        let t = time_point("2027-04-20-13-45-00").unwrap();
        let result = t.sub_minutes(46);

        assert_eq!(result.hour, 12);
        assert_eq!(result.minute, 59);
    }

    #[test]
    fn sub_multiple_hours() {
        let t = time_point("2027-04-20-13-00-00").unwrap();
        let result = t.sub_hours(14);

        assert_eq!(result.day, 19);
        assert_eq!(result.hour, 23);
    }

    #[test]
    fn sub_multiple_days() {
        let t = time_point("2027-04-10").unwrap();
        let result = t.sub_days(10);

        assert_eq!(result.month, 3);
        assert_eq!(result.day, 31);
    }

    #[test]
    fn sub_one_month_rolls_to_previous_year() {
        let t = time_point("2027-01").unwrap();
        let result = TimePoint::sub_one_month(&t);

        assert_eq!(result.year, 2026);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 1);
    }

    #[test]
    fn sub_one_year_decrements_year() {
        let t = time_point("2027").unwrap();
        let result = TimePoint::sub_one_year(&t);

        assert_eq!(result.year, 2026);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);
    }
}

