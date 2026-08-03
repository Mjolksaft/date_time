use date_time::duration::Duration;
use date_time::precision::Precision;
use date_time::time_point::{TimePoint, time_point};

#[test]
fn zero_duration() {
    let d = Duration::zero();

    assert_eq!(d.milliseconds(), 0);
    assert_eq!(d.seconds(), 0);
    assert!(d.is_zero());
    assert!(!d.is_negative());
    assert!(!d.is_positive());
}

#[test]
fn positive_duration() {
    let d = Duration::from_seconds(5);

    assert_eq!(d.seconds(), 5);
    assert_eq!(d.milliseconds(), 5000);
    assert!(!d.is_zero());
    assert!(!d.is_negative());
    assert!(d.is_positive());
}

#[test]
fn negative_duration() {
    let d = Duration::from_seconds(-5);

    assert_eq!(d.seconds(), -5);
    assert_eq!(d.milliseconds(), -5000);
    assert!(d.is_negative());
    assert!(!d.is_positive());
}

#[test]
fn milliseconds_constructors_and_accessors() {
    let d = Duration::from_milliseconds(1500);

    assert_eq!(d.milliseconds(), 1500);
    assert_eq!(d.seconds(), 1);
    assert_eq!(d.subsec_milliseconds(), 500);

    let neg = Duration::from_milliseconds(-1500);
    assert_eq!(neg.milliseconds(), -1500);
    assert_eq!(neg.seconds(), -1);
    assert_eq!(neg.subsec_milliseconds(), -500);
}

#[test]
fn minutes_and_hours_constructors() {
    assert_eq!(Duration::from_minutes(2).seconds(), 120);
    assert_eq!(Duration::from_hours(2).milliseconds(), 7_200_000);
    assert_eq!(Duration::from_hours(-1).seconds(), -3600);
}

#[test]
fn duration_addition() {
    assert_eq!(
        Duration::from_seconds(2) + Duration::from_seconds(3),
        Duration::from_seconds(5)
    );
    assert_eq!(
        Duration::from_milliseconds(1500) + Duration::from_milliseconds(1500),
        Duration::from_seconds(3)
    );
    assert_eq!(
        Duration::from_seconds(1) + Duration::from_seconds(-1),
        Duration::zero()
    );
    assert_eq!(
        Duration::from_milliseconds(500) + Duration::from_seconds(-1),
        Duration::from_milliseconds(-500)
    );
}

#[test]
fn duration_subtraction() {
    assert_eq!(
        Duration::from_seconds(5) - Duration::from_seconds(2),
        Duration::from_seconds(3)
    );
    assert_eq!(
        Duration::from_milliseconds(500) - Duration::from_milliseconds(1500),
        Duration::from_seconds(-1)
    );
    assert_eq!(
        Duration::from_seconds(2) - Duration::from_seconds(2),
        Duration::zero()
    );
}

#[test]
fn duration_negation() {
    assert_eq!(-Duration::from_seconds(5), Duration::from_seconds(-5));
    assert_eq!(-Duration::from_seconds(-5), Duration::from_seconds(5));
    assert_eq!(-Duration::zero(), Duration::zero());
}

#[test]
fn duration_equality_and_ordering() {
    assert_eq!(Duration::from_seconds(2), Duration::from_milliseconds(2000));
    assert!(Duration::from_seconds(3) > Duration::from_seconds(2));
    assert!(Duration::from_seconds(-1) < Duration::zero());
    assert!(Duration::from_seconds(-1) < Duration::from_seconds(1));
}

#[test]
fn adds_duration_within_second() {
    let t = time_point("2027-04-20-13-45-30-000").unwrap();

    assert_eq!(
        t + Duration::from_milliseconds(500),
        time_point("2027-04-20-13-45-30-500").unwrap()
    );
}

#[test]
fn adds_duration_across_second_boundary() {
    let t = time_point("2027-04-20-13-45-30-900").unwrap();

    assert_eq!(
        t + Duration::from_milliseconds(500),
        time_point("2027-04-20-13-45-31-400").unwrap()
    );
}

#[test]
fn adds_duration_across_minute_boundary() {
    let t = time_point("2027-04-20-13-45-59").unwrap();

    assert_eq!(
        t + Duration::from_seconds(2),
        time_point("2027-04-20-13-46-01").unwrap()
    );
}

#[test]
fn adds_duration_across_hour_boundary() {
    let t = time_point("2027-04-20-13-59-59").unwrap();

    assert_eq!(
        t + Duration::from_minutes(1),
        time_point("2027-04-20-14-00-59").unwrap()
    );
}

#[test]
fn adds_duration_across_day_boundary() {
    let t = time_point("2027-04-20-23-59-59").unwrap();

    assert_eq!(
        t + Duration::from_seconds(2),
        time_point("2027-04-21-00-00-01").unwrap()
    );
}

#[test]
fn adds_duration_across_month_boundary() {
    let t = time_point("2027-04-30-23-59-59").unwrap();

    assert_eq!(
        t + Duration::from_seconds(2),
        time_point("2027-05-01-00-00-01").unwrap()
    );
}

#[test]
fn adds_duration_across_year_boundary() {
    let t = time_point("2027-12-31-23-59-59").unwrap();

    assert_eq!(
        t + Duration::from_seconds(2),
        time_point("2028-01-01-00-00-01").unwrap()
    );
}

#[test]
fn adds_duration_negative() {
    let t = time_point("2027-04-20-13-45-30-000").unwrap();

    assert_eq!(
        t.clone() + Duration::from_seconds(-2),
        time_point("2027-04-20-13-45-28-000").unwrap()
    );
    assert_eq!(
        t + Duration::from_milliseconds(-1500),
        time_point("2027-04-20-13-45-28-500").unwrap()
    );
}

#[test]
fn adds_zero_duration_is_identity() {
    let t = time_point("2027-04-20-13-45-30-250").unwrap();

    assert_eq!(t.clone() + Duration::zero(), t);
}

#[test]
fn adds_duration_across_leap_second() {
    let t = time_point("2016-12-31-23-59-59").unwrap();

    assert_eq!(
        t.clone() + Duration::from_seconds(1),
        time_point("2016-12-31-23-59-60").unwrap()
    );
    assert_eq!(
        t + Duration::from_seconds(2),
        time_point("2017-01-01-00-00-00").unwrap()
    );
}

#[test]
fn subtracts_duration_within_second() {
    let t = time_point("2027-04-20-13-45-30-500").unwrap();

    assert_eq!(
        t - Duration::from_milliseconds(200),
        time_point("2027-04-20-13-45-30-300").unwrap()
    );
}

#[test]
fn subtracts_duration_across_second_boundary() {
    let t = time_point("2027-04-20-13-45-30-200").unwrap();

    assert_eq!(
        t - Duration::from_milliseconds(500),
        time_point("2027-04-20-13-45-29-700").unwrap()
    );
}

#[test]
fn subtracts_duration_across_day_boundary() {
    let t = time_point("2027-04-20-00-00-00").unwrap();

    assert_eq!(
        t - Duration::from_seconds(1),
        time_point("2027-04-19-23-59-59").unwrap()
    );
}

#[test]
fn subtracts_duration_negative() {
    let t = time_point("2027-04-20-13-45-30").unwrap();

    assert_eq!(
        t - Duration::from_seconds(-2),
        time_point("2027-04-20-13-45-32").unwrap()
    );
}

#[test]
fn subtracts_duration_across_leap_second() {
    let t = time_point("2017-01-01-00-00-00").unwrap();

    assert_eq!(
        t.clone() - Duration::from_seconds(1),
        time_point("2016-12-31-23-59-60").unwrap()
    );
    assert_eq!(
        t - Duration::from_seconds(2),
        time_point("2016-12-31-23-59-59").unwrap()
    );
}

#[test]
fn duration_preserves_precision() {
    let t = time_point("2027-04-20-13").unwrap();
    let result = t + Duration::from_minutes(30);

    assert_eq!(result.precision, Precision::Hour);
    assert_eq!(result.hour, 13);
    assert_eq!(result.minute, 30);

    let day_point = time_point("2027-04-20").unwrap();
    let shifted = day_point + Duration::from_hours(24);
    assert_eq!(shifted.precision, Precision::Day);
    assert_eq!(shifted.day, 21);
}

#[test]
fn duration_since_returns_positive_for_later() {
    let earlier = time_point("2027-04-20-13-45-00").unwrap();
    let later = time_point("2027-04-20-13-45-05").unwrap();

    assert_eq!(later.duration_since(&earlier), Duration::from_seconds(5));
}

#[test]
fn duration_since_returns_negative_for_earlier() {
    let earlier = time_point("2027-04-20-13-45-00").unwrap();
    let later = time_point("2027-04-20-13-45-05").unwrap();

    assert_eq!(earlier.duration_since(&later), Duration::from_seconds(-5));
}

#[test]
fn time_point_subtraction_operator() {
    let a = time_point("2027-04-20-13-45-30").unwrap();
    let b = time_point("2027-04-20-13-45-10").unwrap();

    assert_eq!(a.clone() - b.clone(), Duration::from_seconds(20));
    assert_eq!(b - a.clone(), Duration::from_seconds(-20));
    assert_eq!(a.clone() - a.clone(), Duration::zero());
}

#[test]
fn time_point_subtraction_with_milliseconds() {
    let a = time_point("2027-04-20-13-45-30-500").unwrap();
    let b = time_point("2027-04-20-13-45-30-200").unwrap();

    assert_eq!(a.clone() - b.clone(), Duration::from_milliseconds(300));
    assert_eq!(b - a, Duration::from_milliseconds(-300));
}

#[test]
fn time_point_subtraction_across_boundaries() {
    let a = time_point("2028-01-01-00-00-00").unwrap();
    let b = time_point("2027-12-31-23-59-59").unwrap();

    assert_eq!(a - b, Duration::from_seconds(1));
}

#[test]
fn time_point_subtraction_across_leap_second() {
    let before = time_point("2016-12-31-23-59-59").unwrap();
    let after = time_point("2017-01-01-00-00-00").unwrap();

    assert_eq!(after.clone() - before.clone(), Duration::from_seconds(2));
    assert_eq!(before - after, Duration::from_seconds(-2));
}

#[test]
fn time_point_subtraction_with_leap_second_operand() {
    let leap = time_point("2016-12-31-23-59-60").unwrap();
    let before = time_point("2016-12-31-23-59-59").unwrap();
    let after = time_point("2017-01-01-00-00-00").unwrap();

    assert_eq!(leap.clone() - before, Duration::from_seconds(1));
    assert_eq!(after - leap, Duration::from_seconds(1));
}

#[test]
fn round_trip_add_then_subtract() {
    let t = time_point("2027-04-20-13-45-30-250").unwrap();
    let d = Duration::from_milliseconds(1750);

    assert_eq!(t.clone() + d - d, t);
}

#[test]
fn round_trip_subtract_then_add() {
    let t = time_point("2027-04-20-13-45-30-250").unwrap();
    let d = Duration::from_seconds(90);

    assert_eq!(t.clone() - d + d, t);
}

#[test]
fn round_trip_with_negative_duration() {
    let t = time_point("2027-04-20-13-45-30-250").unwrap();
    let d = Duration::from_seconds(-90);

    assert_eq!(t.clone() + d - d, t);
    assert_eq!(t.clone() - d + d, t);
}

#[test]
fn round_trip_across_leap_second() {
    let t = time_point("2016-12-31-23-59-55").unwrap();
    let d = Duration::from_seconds(10);

    let result = t.clone() + d;
    assert_eq!(result, time_point("2017-01-01-00-00-04").unwrap());
    assert_eq!(result - d, t);
}

#[test]
fn duration_difference_recreates_the_duration() {
    let t = time_point("2027-04-20-13-45-30-250").unwrap();
    let d = Duration::from_milliseconds(1750);
    let shifted = t.clone() + d;

    assert_eq!(shifted - t, d);
}

#[test]
fn duration_display() {
    assert_eq!(Duration::zero().to_string(), "0s");
    assert_eq!(Duration::from_seconds(5).to_string(), "5s");
    assert_eq!(Duration::from_seconds(-5).to_string(), "-5s");
    assert_eq!(Duration::from_milliseconds(1500).to_string(), "1.500s");
    assert_eq!(Duration::from_milliseconds(-1500).to_string(), "-1.500s");
}
