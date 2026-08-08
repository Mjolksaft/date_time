use date_time::period::Period;
use date_time::time_point::{TimePoint, time_point};

fn point(value: &str) -> TimePoint {
    time_point(value).unwrap()
}

fn assert_add(base: &str, period: Period, expected: &str) {
    assert_eq!(
        point(base).add_period(period),
        point(expected),
        "{base} + {period}"
    );
}

#[test]
fn add_months_clamps_to_end_of_month() {
    assert_add("2027-01-31", Period::from_months(1), "2027-02-28");
    assert_add("2027-01-31", Period::from_months(2), "2027-03-31");
}

#[test]
fn add_months_rolls_over_years() {
    assert_add("2027-12-15", Period::from_months(1), "2028-01-15");
    assert_add("2027-11-30", Period::from_months(3), "2028-02-29");
}

#[test]
fn add_years_clamps_leap_day() {
    assert_add("2024-02-29", Period::from_years(1), "2025-02-28");
    assert_add("2024-02-29", Period::from_years(4), "2028-02-29");
}

#[test]
fn add_months_backwards() {
    assert_add("2027-03-31", Period::from_months(-1), "2027-02-28");
    assert_add("2027-01-15", Period::from_months(-1), "2026-12-15");
}

#[test]
fn add_days_preserves_time_of_day() {
    let result = point("2027-01-01-13-45-30").add_period(Period::from_days(10));

    assert_eq!(result, point("2027-01-11-13-45-30"));
}

#[test]
fn add_days_backwards() {
    assert_add("2027-01-01", Period::from_days(-1), "2026-12-31");
}

#[test]
fn add_combined_period() {
    assert_add(
        "2027-01-31-00",
        Period::new(1, 1, 10, 3, 0, 0, 0),
        "2028-03-10-03",
    );
}

#[test]
fn add_fixed_time_part() {
    assert_add(
        "2027-04-20-13-45-30-500",
        Period::from_seconds(90),
        "2027-04-20-13-47-00-500",
    );
}

#[test]
fn sub_period_is_inverse_of_add() {
    let t = point("2027-04-20-13-45-30");
    let p = Period::new(1, 2, 3, 4, 5, 6, 7);

    assert_eq!(t.add_period(p).sub_period(p), t);
    assert_eq!(t.sub_period(p).add_period(p), t);
}

#[test]
fn operators_add_and_sub_period() {
    let t = point("2027-04-20-13-45-30");
    let p = Period::from_months(2);

    assert_eq!(t.clone() + p, point("2027-06-20-13-45-30"));
    assert_eq!(t.clone() - p, point("2027-02-20-13-45-30"));
}

#[test]
fn period_addition_subtraction_negation() {
    let a = Period::new(1, 2, 3, 0, 0, 0, 0);
    let b = Period::new(0, 1, 1, 0, 0, 0, 0);

    assert_eq!(a + b, Period::new(1, 3, 4, 0, 0, 0, 0));
    assert_eq!(a - b, Period::new(1, 1, 2, 0, 0, 0, 0));
    assert_eq!(-a, Period::new(-1, -2, -3, 0, 0, 0, 0));
    assert_eq!(a + (-a), Period::zero());
}

#[test]
fn months_normalize_into_years() {
    assert_eq!(Period::from_months(13), Period::new(1, 1, 0, 0, 0, 0, 0));
    assert_eq!(Period::from_months(-13), Period::new(-1, -1, 0, 0, 0, 0, 0));
    assert_eq!(
        Period::new(1, -3, 0, 0, 0, 0, 0),
        Period::new(0, 9, 0, 0, 0, 0, 0)
    );
}

#[test]
fn time_components_normalize() {
    assert_eq!(
        Period::from_milliseconds(1500),
        Period::new(0, 0, 0, 0, 0, 1, 500)
    );
    assert_eq!(Period::from_seconds(90), Period::new(0, 0, 0, 0, 1, 30, 0));
}

#[test]
fn accessors_and_flags() {
    let p = Period::new(1, 2, 3, 4, 5, 6, 7);

    assert_eq!((p.years(), p.months(), p.days()), (1, 2, 3));
    assert_eq!(
        (p.hours(), p.minutes(), p.seconds(), p.milliseconds()),
        (4, 5, 6, 7)
    );
    assert!(!p.is_zero());
    assert!(!p.is_negative());
    assert!(Period::zero().is_zero());
    assert!(Period::from_days(-1).is_negative());
    assert!(Period::new(-1, 1, 0, 0, 0, 0, 0).is_negative());
}

#[test]
fn display_iso_style() {
    assert_eq!(Period::zero().to_string(), "P0D");
    assert_eq!(
        Period::new(1, 2, 3, 4, 5, 6, 0).to_string(),
        "P1Y2M3DT4H5M6S"
    );
    assert_eq!(Period::new(0, 0, 0, 0, 0, 1, 500).to_string(), "PT1.500S");
    assert_eq!(Period::from_years(-1).to_string(), "-P1Y");
    assert_eq!(Period::from_days(10).to_string(), "P10D");
}

#[test]
fn precision_preserved() {
    let result = point("2027-04-20-13").add_period(Period::from_days(2));

    assert_eq!(result, point("2027-04-22-13"));
}
