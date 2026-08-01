use date_time::precision::Precision;
use date_time::time_point::{TimePoint, time_point};
use date_time::time_zone::TimeZone;

#[test]
fn parses_year_point() {
    let result = time_point("2027").unwrap();

    assert_eq!(
        result,
        TimePoint {
            year: 2027,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
            precision: Precision::Year,
            zone: TimeZone::UTC,
            uncertainty: None,
        }
    );
}

#[test]
fn parses_month_point() {
    let result = time_point("2027-11").unwrap();

    assert_eq!(
        result,
        TimePoint {
            year: 2027,
            month: 11,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
            precision: Precision::Month,
            zone: TimeZone::UTC,
            uncertainty: None,
        }
    );
}

#[test]
fn parses_day_point() {
    let result = time_point("2027-04-20").unwrap();

    assert_eq!(
        result,
        TimePoint {
            year: 2027,
            month: 4,
            day: 20,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
            precision: Precision::Day,
            zone: TimeZone::UTC,
            uncertainty: None,
        }
    );
}

#[test]
fn parses_hour_point() {
    let result = time_point("2027-04-20-13").unwrap();

    assert_eq!(
        result,
        TimePoint {
            year: 2027,
            month: 4,
            day: 20,
            hour: 13,
            minute: 0,
            second: 0,
            millisecond: 0,
            precision: Precision::Hour,
            zone: TimeZone::UTC,
            uncertainty: None,
        }
    );
}

#[test]
fn parses_minute_point() {
    let result = time_point("2027-04-20-13-45").unwrap();

    assert_eq!(
        result,
        TimePoint {
            year: 2027,
            month: 4,
            day: 20,
            hour: 13,
            minute: 45,
            second: 0,
            millisecond: 0,
            precision: Precision::Minute,
            zone: TimeZone::UTC,
            uncertainty: None,
        }
    );
}

#[test]
fn parses_second_point() {
    let result = time_point("2027-04-20-13-45-30").unwrap();

    assert_eq!(
        result,
        TimePoint {
            year: 2027,
            month: 4,
            day: 20,
            hour: 13,
            minute: 45,
            second: 30,
            millisecond: 0,
            precision: Precision::Second,
            zone: TimeZone::UTC,
            uncertainty: None,
        }
    );
}

#[test]
fn fails_on_invalid_month() {
    let result = time_point("2027-13-01");
    assert!(result.is_err());
}

#[test]
fn fails_on_invalid_day() {
    let result = time_point("2027-04-40");
    assert!(result.is_err());
}

#[test]
fn fails_on_invalid_hour() {
    let result = time_point("2027-04-20-24");
    assert!(result.is_err());
}

#[test]
fn fails_on_invalid_minute() {
    let result = time_point("2027-04-20-13-60");
    assert!(result.is_err());
}

#[test]
fn fails_on_invalid_second() {
    let result = time_point("2027-04-20-13-45-60");
    assert!(result.is_err());
}

#[test]
fn fails_on_too_many_parts() {
    let result = time_point("2027-04-10-12-30-45-990-extra");
    assert!(result.is_err());
}

#[test]
fn fails_on_empty_input() {
    let result = time_point("");
    assert!(result.is_err());
}

#[test]
fn fails_on_non_numeric_year() {
    let result = time_point("abcd");
    assert!(result.is_err());
}
