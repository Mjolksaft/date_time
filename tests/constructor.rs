use date_time::precision::Precision;
use date_time::time_point::TimePoint;
use date_time::time_zone::TimeZone;

#[test]
fn now_utc_returns_millisecond_precision() {
    let now = TimePoint::now_utc();

    println!("{:?}", now);
    assert_eq!(now.precision, Precision::Millisecond);
    assert!(now.month >= 1 && now.month <= 12);
    assert!(now.day >= 1 && now.day <= 31);
    assert!(now.hour <= 23);
    assert!(now.minute <= 59);
    assert!(now.second <= 59);
    assert!(now.millisecond <= 999);
}

#[test]
fn constructs_year_precision_with_defaults() {
    let result = TimePoint::new(2027, None, None, None, None, None, None).unwrap();

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
fn constructs_month_precision_with_defaults() {
    let result = TimePoint::new(2027, Some(4), None, None, None, None, None).unwrap();

    assert_eq!(
        result,
        TimePoint {
            year: 2027,
            month: 4,
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
fn constructs_day_precision_with_defaults() {
    let result = TimePoint::new(2027, Some(4), Some(20), None, None, None, None).unwrap();

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
fn constructs_hour_precision_with_defaults() {
    let result = TimePoint::new(2027, Some(4), Some(20), Some(13), None, None, None).unwrap();

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
fn constructs_minute_precision_with_defaults() {
    let result = TimePoint::new(2027, Some(4), Some(20), Some(13), Some(45), None, None).unwrap();

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
fn constructs_second_precision() {
    let result =
        TimePoint::new(2027, Some(4), Some(20), Some(13), Some(45), Some(30), None).unwrap();

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
fn rejects_day_without_month() {
    let result = TimePoint::new(2027, None, Some(20), None, None, None, None);
    assert!(result.is_err());
}

#[test]
fn rejects_hour_without_day() {
    let result = TimePoint::new(2027, Some(4), None, Some(13), None, None, None);
    assert!(result.is_err());
}

#[test]
fn rejects_minute_without_hour() {
    let result = TimePoint::new(2027, Some(4), Some(20), None, Some(45), None, None);
    assert!(result.is_err());
}

#[test]
fn rejects_second_without_minute() {
    let result = TimePoint::new(2027, Some(4), Some(20), Some(13), None, Some(30), None);
    assert!(result.is_err());
}

#[test]
fn rejects_invalid_month() {
    let result = TimePoint::new(2027, Some(13), None, None, None, None, None);
    assert!(result.is_err());
}

#[test]
fn rejects_invalid_day() {
    let result = TimePoint::new(2027, Some(4), Some(40), None, None, None, None);
    assert!(result.is_err());
}

#[test]
fn rejects_invalid_hour() {
    let result = TimePoint::new(2027, Some(4), Some(20), Some(24), None, None, None);
    assert!(result.is_err());
}

#[test]
fn rejects_invalid_minute() {
    let result = TimePoint::new(2027, Some(4), Some(20), Some(13), Some(60), None, None);
    assert!(result.is_err());
}

#[test]
fn rejects_invalid_second() {
    let result = TimePoint::new(2027, Some(4), Some(20), Some(13), Some(45), Some(60), None);

    assert!(result.is_err());
}

#[test]
fn supports_leap_year_date() {
    let result = TimePoint::new(2028, Some(2), Some(29), None, None, None, None);
    assert!(result.is_ok());
}

#[test]
fn rejects_invalid_non_leap_year_date() {
    let result = TimePoint::new(2027, Some(2), Some(29), None, None, None, None);
    assert!(result.is_err());
}
