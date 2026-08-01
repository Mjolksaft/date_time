use date_time::precision::Precision;
use date_time::time_point::{TimePoint, time_point};

#[test]
fn subtracts_one_second() {
    let t = time_point("2027-04-20-13-45-30").unwrap();
    let result = t.sub_seconds(1);

    assert_eq!(result.second, 29);
}

#[test]
fn subtracts_second_with_minute_rollover() {
    let t = time_point("2027-04-20-13-45-00").unwrap();
    let result = t.sub_seconds(1);

    assert_eq!(result.minute, 44);
    assert_eq!(result.second, 59);
}

#[test]
fn subtracts_second_with_day_rollover() {
    let t = time_point("2027-04-20-00-00-00").unwrap();
    let result = t.sub_seconds(1);

    assert_eq!(result.day, 19);
    assert_eq!(result.hour, 23);
    assert_eq!(result.minute, 59);
    assert_eq!(result.second, 59);
}

#[test]
fn adds_one_second() {
    let t = time_point("2027-04-20-13-45-30").unwrap();
    let result = t.add_seconds(1);

    assert_eq!(result.second, 31);
}

#[test]
fn second_rolls_to_next_minute() {
    let t = time_point("2027-04-20-13-45-59").unwrap();
    let result = t.add_seconds(1);

    assert_eq!(result.minute, 46);
    assert_eq!(result.second, 0);
}

#[test]
fn minute_rolls_to_next_hour() {
    let t = time_point("2027-04-20-13-59-59").unwrap();
    let result = t.add_seconds(1);

    assert_eq!(result.hour, 14);
    assert_eq!(result.minute, 0);
    assert_eq!(result.second, 0);
}

#[test]
fn hour_rolls_to_next_day() {
    let t = time_point("2027-04-20-23-59-59").unwrap();
    let result = t.add_seconds(1);

    assert_eq!(result.year, 2027);
    assert_eq!(result.month, 4);
    assert_eq!(result.day, 21);
    assert_eq!(result.hour, 0);
    assert_eq!(result.minute, 0);
    assert_eq!(result.second, 0);
}

#[test]
fn day_rolls_to_next_month() {
    let t = time_point("2027-04-30-23-59-59").unwrap();
    let result = t.add_seconds(1);

    assert_eq!(result.year, 2027);
    assert_eq!(result.month, 5);
    assert_eq!(result.day, 1);
}

#[test]
fn month_rolls_to_next_year() {
    let t = time_point("2027-12-31-23-59-59").unwrap();
    let result = t.add_seconds(1);

    assert_eq!(result.year, 2028);
    assert_eq!(result.month, 1);
    assert_eq!(result.day, 1);
}

#[test]
fn adds_minutes() {
    let t = time_point("2027-04-20-13-45-30").unwrap();
    let result = t.add_minutes(2);

    assert_eq!(result.hour, 13);
    assert_eq!(result.minute, 47);
    assert_eq!(result.second, 30);
}

#[test]
fn adds_hours() {
    let t = time_point("2027-04-20-13-45-30").unwrap();
    let result = t.add_hours(2);

    assert_eq!(result.hour, 15);
    assert_eq!(result.minute, 45);
    assert_eq!(result.second, 30);
}

#[test]
fn arithmetic_preserves_precision() {
    let t = time_point("2027-04-20-13").unwrap();
    let result = t.add_seconds(30);

    assert_eq!(result.precision, Precision::Hour);
}

#[test]
fn subtracting_from_after_leap_second_returns_60() {
    let t = time_point("2017-01-01-00-00-00").unwrap();
    let result = TimePoint::sub_one_second(&t);

    assert_eq!(result.year, 2016);
    assert_eq!(result.month, 12);
    assert_eq!(result.day, 31);
    assert_eq!(result.hour, 23);
    assert_eq!(result.minute, 59);
    assert_eq!(result.second, 60);
}
