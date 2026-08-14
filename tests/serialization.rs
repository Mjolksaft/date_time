#![cfg(feature = "serde")]

use date_time::duration::Duration;
use date_time::interval::{AllenRelation, interval, to_interval};
use date_time::period::Period;
use date_time::precision::Precision;
use date_time::time_point::time_point;
use date_time::time_zone::TimeZone;
use date_time::truth_values::TruthValue;
use date_time::uncertainty::Uncertainty;

fn round_trip<T>(value: &T) -> T
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug,
{
    let json = serde_json::to_string(value).expect("serialize");
    serde_json::from_str(&json).expect("deserialize")
}

#[test]
fn serialize_time_point_round_trip() {
    let t = time_point("2027-04-20-13-45-30-250")
        .unwrap()
        .with_zone(TimeZone::fixed(5, 30).unwrap())
        .with_uncertainty(Uncertainty::from_seconds(10));

    let back: date_time::time_point::TimePoint = round_trip(&t);

    assert_eq!(back, t);
}

#[test]
fn serialize_precision_and_zone() {
    assert_eq!(round_trip(&Precision::Millisecond), Precision::Millisecond);
    assert_eq!(
        round_trip(&TimeZone::Fixed {
            hours: -5,
            minutes: 0
        }),
        TimeZone::Fixed {
            hours: -5,
            minutes: 0
        }
    );
    assert_eq!(round_trip(&TimeZone::UTC), TimeZone::UTC);
}

#[test]
fn serialize_duration_and_period() {
    let d = Duration::from_milliseconds(-12345);
    assert_eq!(round_trip(&d), d);

    let p = Period::new(2, 14, 3, 90, 0, 0, 1500);
    assert_eq!(round_trip(&p), p);
}

#[test]
fn serialize_interval_round_trip() {
    let iv = interval(
        &time_point("2027-04-20-00-00-00").unwrap(),
        &time_point("2027-04-21-00-00-00").unwrap(),
    )
    .unwrap();

    let back: date_time::interval::Interval = round_trip(&iv);

    assert_eq!(back.lower_key, iv.lower_key);
    assert_eq!(back.upper_key, iv.upper_key);
    assert_eq!(back.allen_relation(&iv).unwrap(), AllenRelation::Equal);
}

#[test]
fn serialize_uncertainty_and_truth_value() {
    assert_eq!(
        round_trip(&Uncertainty::from_seconds(30)),
        Uncertainty::from_seconds(30)
    );
    assert_eq!(round_trip(&TruthValue::Unknown), TruthValue::Unknown);
}

#[test]
fn serialize_to_interval_symmetric() {
    let p = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(5));

    let iv = to_interval(&p, None).unwrap();
    let back: date_time::interval::Interval = round_trip(&iv);

    assert_eq!(back.lower_key, iv.lower_key);
    assert_eq!(back.upper_key, iv.upper_key);
}

#[test]
fn json_shape_is_readable() {
    let json = serde_json::to_string(
        &time_point("2027-04-20-13-45-30")
            .unwrap()
            .with_uncertainty(Uncertainty::from_seconds(2)),
    )
    .unwrap();

    assert!(json.contains("\"year\":2027"));
    assert!(json.contains("\"hour\":13"));
    assert!(json.contains("\"seconds\":2"));
}
