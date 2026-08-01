use date_time::interval::{Interval, to_interval};
use date_time::precision::Precision;
use date_time::time_point::{TimePoint, time_point};
use date_time::time_zone::TimeZone;

#[test]
fn normalizes_year_to_interval() {
    let result = to_interval(&time_point("2027").unwrap(), None).unwrap();

    assert_eq!(
        result,
        Interval::new(
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
            },
            TimePoint {
                year: 2028,
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
        )
    );
}

#[test]
fn normalizes_hour_to_interval() {
    let result = to_interval(&time_point("2027-04-20-13").unwrap(), None).unwrap();

    assert_eq!(
        result,
        Interval::new(
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
            },
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 14,
                minute: 0,
                second: 0,
                millisecond: 0,
                precision: Precision::Hour,
                zone: TimeZone::UTC,
                uncertainty: None,
            }
        )
    );
}

#[test]
fn normalizes_minute_to_interval() {
    let result = to_interval(&time_point("2027-04-20-13-45").unwrap(), None).unwrap();

    assert_eq!(
        result,
        Interval::new(
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
            },
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 13,
                minute: 46,
                second: 0,
                millisecond: 0,
                precision: Precision::Minute,
                zone: TimeZone::UTC,
                uncertainty: None,
            }
        )
    );
}

#[test]
fn normalizes_second_to_interval() {
    let result = to_interval(&time_point("2027-04-20-13-45-30").unwrap(), None).unwrap();

    assert_eq!(
        result,
        Interval::new(
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
            },
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 13,
                minute: 45,
                second: 31,
                millisecond: 0,
                precision: Precision::Second,
                zone: TimeZone::UTC,
                uncertainty: None,
            }
        )
    );
}
