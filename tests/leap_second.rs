use date_time::leap_second::get_leap_seconds_data;
use date_time::time_point::{TimePoint, time_point};

#[test]
fn get_leap_seconds_data_test() {
    println!("{:?}", get_leap_seconds_data());
}

#[test]
fn leap_second_goes_from_59_to_60() {
    let t = time_point("2016-12-31-23-59-59").unwrap();
    let result = TimePoint::add_one_second(&t);

    assert_eq!(result.year, 2016);
    assert_eq!(result.month, 12);
    assert_eq!(result.day, 31);
    assert_eq!(result.hour, 23);
    assert_eq!(result.minute, 59);
    assert_eq!(result.second, 60);
}

#[test]
fn leap_second_goes_from_60_to_next_day() {
    let t = time_point("2016-12-31-23-59-60").unwrap();
    let result = TimePoint::add_one_second(&t);

    assert_eq!(result.year, 2017);
    assert_eq!(result.month, 1);
    assert_eq!(result.day, 1);
    assert_eq!(result.hour, 0);
    assert_eq!(result.minute, 0);
    assert_eq!(result.second, 0);
}

#[test]
fn normal_day_goes_from_59_to_next_day() {
    let t = time_point("2027-12-31-23-59-59").unwrap();
    let result = TimePoint::add_one_second(&t);

    assert_eq!(result.year, 2028);
    assert_eq!(result.month, 1);
    assert_eq!(result.day, 1);
    assert_eq!(result.hour, 0);
    assert_eq!(result.minute, 0);
    assert_eq!(result.second, 0);
}

#[test]
fn add_seconds_includes_leap_second() {
    let t = time_point("2016-12-31-23-59-59").unwrap();
    let result = t.add_seconds(2);

    assert_eq!(result.year, 2017);
    assert_eq!(result.month, 1);
    assert_eq!(result.day, 1);
    assert_eq!(result.hour, 0);
    assert_eq!(result.minute, 0);
    assert_eq!(result.second, 0);
}

#[test]
fn rejects_leap_second_on_non_leap_second_day() {
    let result = time_point("2027-12-31-23-59-60");
    assert!(result.is_err());
}

#[test]
fn rejects_leap_second_at_wrong_hour() {
    let result = time_point("2016-12-31-22-59-60");
    assert!(result.is_err());
}

#[test]
fn rejects_leap_second_at_wrong_minute() {
    let result = time_point("2016-12-31-23-58-60");
    assert!(result.is_err());
}

#[test]
fn sub_seconds_from_after_leap_second_returns_60() {
    let t = time_point("2017-01-01-00-00-00").unwrap();
    let result = t.sub_seconds(1);

    assert_eq!(result.year, 2016);
    assert_eq!(result.month, 12);
    assert_eq!(result.day, 31);
    assert_eq!(result.hour, 23);
    assert_eq!(result.minute, 59);
    assert_eq!(result.second, 60);
}

#[test]
fn sub_seconds_from_leap_second_returns_59() {
    let t = time_point("2016-12-31-23-59-60").unwrap();
    let result = t.sub_seconds(1);

    assert_eq!(result.year, 2016);
    assert_eq!(result.month, 12);
    assert_eq!(result.day, 31);
    assert_eq!(result.hour, 23);
    assert_eq!(result.minute, 59);
    assert_eq!(result.second, 59);
}

#[test]
fn sub_seconds_normal_day_rollover() {
    let t = time_point("2027-01-01-00-00-00").unwrap();
    let result = t.sub_seconds(1);

    assert_eq!(result.year, 2026);
    assert_eq!(result.month, 12);
    assert_eq!(result.day, 31);
    assert_eq!(result.hour, 23);
    assert_eq!(result.minute, 59);
    assert_eq!(result.second, 59);
}
