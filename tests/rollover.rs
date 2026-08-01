use date_time::interval::to_interval;
use date_time::time_point::time_point;

#[test]
fn hour_rollover_to_next_day() {
    let result = to_interval(&time_point("2027-04-20-23").unwrap(), None).unwrap();

    assert_eq!(result.upper.hour, 0);
    assert_eq!(result.upper.day, 21);
}

#[test]
fn minute_rollover_to_next_hour() {
    let result = to_interval(&time_point("2027-04-20-13-59").unwrap(), None).unwrap();

    assert_eq!(result.upper.hour, 14);
    assert_eq!(result.upper.minute, 0);
}

#[test]
fn second_rollover_to_next_minute() {
    let result = to_interval(&time_point("2027-04-20-13-45-59").unwrap(), None).unwrap();

    assert_eq!(result.upper.minute, 46);
    assert_eq!(result.upper.second, 0);
}
