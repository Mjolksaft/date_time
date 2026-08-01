use date_time::time_point::{TimePoint, time_point};
use date_time::unix::{from_unix_timestamp, to_unix_timestamp};

#[test]
fn unix_timestamp_rejects_leap_second() {
    let t = time_point("2016-12-31-23-59-60").unwrap();

    assert!(to_unix_timestamp(&t).is_err());
}

#[test]
fn unix_roundtrip() {
    let t = time_point("2027-04-20-13-45-30").unwrap();

    let ts = to_unix_timestamp(&t).unwrap();
    let back = from_unix_timestamp(ts);

    assert_eq!(t.year, back.year);
}

#[test]
fn time_point_method_delegates_to_unix_module() {
    let t = time_point("2027-04-20-13-45-30").unwrap();

    assert_eq!(t.to_unix_timestamp(), to_unix_timestamp(&t));
    assert_eq!(
        TimePoint::from_unix_timestamp(t.to_unix_timestamp().unwrap()),
        t
    );
}

#[test]
fn known_unix_timestamp() {
    let t = time_point("1970-01-01-00-00-00").unwrap();

    assert_eq!(to_unix_timestamp(&t).unwrap(), 0);
}

#[test]
fn leap_second_timestamp_roundtrip_is_invalid() {
    let t = time_point("2017-01-01-00-00-00").unwrap();

    let ts = to_unix_timestamp(&t).unwrap();
    let back = from_unix_timestamp(ts);

    assert_eq!(back.second, 0);
    assert_eq!(back.day, 1);
}
