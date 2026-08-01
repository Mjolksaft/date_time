use date_time::precision::Precision;
use date_time::tai::{tai_to_utc, tai_utc_offset, utc_to_tai};
use date_time::time_point::{TimePoint, time_point};
use date_time::time_zone::TimeZone;

#[test]
fn utc_to_tai_normal_second() {
    let utc = time_point("2016-12-31-23-59-59").unwrap();
    let tai = utc_to_tai(&utc).unwrap();

    assert_eq!(tai.year, 2017);
    assert_eq!(tai.month, 1);
    assert_eq!(tai.day, 1);
    assert_eq!(tai.hour, 0);
    assert_eq!(tai.minute, 0);
    assert_eq!(tai.second, 35);
    assert_eq!(tai.zone, TimeZone::TAI);
}

#[test]
fn utc_leap_second_absorbs_into_tai() {
    let utc = time_point("2016-12-31-23-59-60").unwrap();
    let tai = utc_to_tai(&utc).unwrap();

    assert_eq!(tai.year, 2017);
    assert_eq!(tai.month, 1);
    assert_eq!(tai.day, 1);
    assert_eq!(tai.hour, 0);
    assert_eq!(tai.minute, 0);
    assert_eq!(tai.second, 36);
    assert_eq!(tai.zone, TimeZone::TAI);
}

#[test]
fn utc_to_tai_after_leap_second() {
    let utc = time_point("2017-01-01-00-00-00").unwrap();
    let tai = utc_to_tai(&utc).unwrap();

    assert_eq!(tai.second, 37);
}

#[test]
fn utc_to_tai_preserves_precision() {
    let utc = time_point("2027-04").unwrap();
    let tai = utc_to_tai(&utc).unwrap();

    assert_eq!(tai.precision, Precision::Month);
    assert_eq!(tai.zone, TimeZone::TAI);
}

#[test]
fn tai_to_utc_normal_second() {
    let tai = TimePoint {
        year: 2017,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 35,
        millisecond: 0,
        precision: Precision::Second,
        zone: TimeZone::TAI,
        uncertainty: None,
    };
    let utc = tai_to_utc(&tai).unwrap();

    assert_eq!(utc.year, 2016);
    assert_eq!(utc.month, 12);
    assert_eq!(utc.day, 31);
    assert_eq!(utc.hour, 23);
    assert_eq!(utc.minute, 59);
    assert_eq!(utc.second, 59);
    assert_eq!(utc.zone, TimeZone::UTC);
}

#[test]
fn tai_second_during_leap_second_maps_to_60() {
    let tai = TimePoint {
        year: 2017,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 36,
        millisecond: 0,
        precision: Precision::Second,
        zone: TimeZone::TAI,
        uncertainty: None,
    };
    let utc = tai_to_utc(&tai).unwrap();

    assert_eq!(utc.year, 2016);
    assert_eq!(utc.month, 12);
    assert_eq!(utc.day, 31);
    assert_eq!(utc.hour, 23);
    assert_eq!(utc.minute, 59);
    assert_eq!(utc.second, 60);
    assert_eq!(utc.zone, TimeZone::UTC);
}

#[test]
fn tai_to_utc_after_leap_second() {
    let tai = TimePoint {
        year: 2017,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 37,
        millisecond: 0,
        precision: Precision::Second,
        zone: TimeZone::TAI,
        uncertainty: None,
    };
    let utc = tai_to_utc(&tai).unwrap();

    assert_eq!(utc.year, 2017);
    assert_eq!(utc.month, 1);
    assert_eq!(utc.day, 1);
    assert_eq!(utc.hour, 0);
    assert_eq!(utc.minute, 0);
    assert_eq!(utc.second, 0);
}

#[test]
fn tai_to_utc_preserves_precision() {
    let tai = time_point("2027-04").unwrap();
    let mut tai = tai;
    tai.zone = TimeZone::TAI;

    let utc = tai_to_utc(&tai).unwrap();

    assert_eq!(utc.precision, Precision::Month);
    assert_eq!(utc.zone, TimeZone::UTC);
}

#[test]
fn utc_tai_roundtrip() {
    let utc = time_point("2027-04-20-13-45-30").unwrap();

    let tai = utc_to_tai(&utc).unwrap();
    let back = tai_to_utc(&tai).unwrap();

    assert_eq!(back, utc);
}

#[test]
fn utc_tai_roundtrip_across_leap_second() {
    let utc = time_point("2016-12-31-23-59-59").unwrap();
    let tai = utc_to_tai(&utc).unwrap();
    let back = tai_to_utc(&tai).unwrap();

    assert_eq!(back, utc);
}

#[test]
fn offset_is_37_after_2017() {
    let t = time_point("2027-04-20-13-45-30").unwrap();
    assert_eq!(tai_utc_offset(&t).unwrap(), 37);
}

#[test]
fn offset_is_36_during_2016() {
    let t = time_point("2016-06-01-00-00-00").unwrap();
    assert_eq!(tai_utc_offset(&t).unwrap(), 36);
}

#[test]
fn offset_is_35_during_2015() {
    let t = time_point("2015-06-01-00-00-00").unwrap();
    assert_eq!(tai_utc_offset(&t).unwrap(), 35);
}

#[test]
fn offset_before_first_entries_defaults() {
    let t = time_point("1971-01-01-00-00-00").unwrap();
    assert_eq!(tai_utc_offset(&t).unwrap(), 10);
}

#[test]
fn offset_for_leap_second_is_still_pre_transition() {
    let t = time_point("2016-12-31-23-59-60").unwrap();
    assert_eq!(tai_utc_offset(&t).unwrap(), 36);
}

#[test]
fn offset_matches_for_tai_point() {
    let utc = time_point("2017-01-01-00-00-00").unwrap();
    let tai = utc_to_tai(&utc).unwrap();

    assert_eq!(tai_utc_offset(&tai).unwrap(), 37);
}

#[test]
fn utc_to_tai_requires_utc_zone() {
    let mut tai = time_point("2027-04-20").unwrap();
    tai.zone = TimeZone::TAI;

    assert!(utc_to_tai(&tai).is_err());
}

#[test]
fn tai_to_utc_requires_tai_zone() {
    let utc = time_point("2027-04-20").unwrap();

    assert!(tai_to_utc(&utc).is_err());
}

#[test]
fn unix_zone_has_no_offset() {
    let mut unix = time_point("2027-04-20").unwrap();
    unix.zone = TimeZone::Unix;

    assert!(tai_utc_offset(&unix).is_err());
}
