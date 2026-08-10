use date_time::interval::{interval, to_interval};
use date_time::time_point::{TimePoint, time_point};
use date_time::truth_values::TruthValue;
use date_time::uncertainty::Uncertainty;

#[test]
fn builder_sets_uncertainty() {
    let point = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));

    assert_eq!(point.uncertainty(), Some(Uncertainty::from_seconds(10)));
    assert_eq!(point.uncertainty().unwrap().seconds(), 10);
}

#[test]
fn default_is_no_uncertainty() {
    let point = time_point("2027-04-20-12-00-00").unwrap();

    assert_eq!(point.uncertainty(), None);
}

#[test]
fn interval_expands_symmetrically_in_seconds() {
    let point = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));
    let result = to_interval(&point, None).unwrap();

    assert_eq!(result.lower, time_point("2027-04-20-11-59-50").unwrap());
    assert_eq!(result.upper, time_point("2027-04-20-12-00-10").unwrap());
}

#[test]
fn interval_expands_across_calendar_boundaries() {
    let point = time_point("2027-04-20")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10 * 24 * 60 * 60));
    let result = to_interval(&point, None).unwrap();

    assert_eq!(result.lower, time_point("2027-04-10").unwrap());
    assert_eq!(result.upper, time_point("2027-04-30").unwrap());
}

#[test]
fn interval_expands_month_precision() {
    let point = time_point("2027-04")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10 * 24 * 60 * 60));
    let result = to_interval(&point, None).unwrap();

    assert_eq!(result.lower.year, 2027);
    assert_eq!(result.lower.month, 3);
    assert_eq!(result.lower.day, 22);
    assert_eq!(result.upper.year, 2027);
    assert_eq!(result.upper.month, 4);
    assert_eq!(result.upper.day, 11);
}

#[test]
fn interval_spans_leap_second() {
    let point = time_point("2016-12-31-23-59-60")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));
    let result = to_interval(&point, None).unwrap();

    assert_eq!(result.lower, time_point("2016-12-31-23-59-50").unwrap());
    assert_eq!(result.upper, time_point("2017-01-01-00-00-09").unwrap());
}

#[test]
fn no_uncertainty_keeps_precision_interval() {
    let point = time_point("2027-04-20-12-00-00").unwrap();
    let result = to_interval(&point, None).unwrap();

    assert_eq!(result.lower, time_point("2027-04-20-12-00-00").unwrap());
    assert_eq!(result.upper, time_point("2027-04-20-12-00-01").unwrap());
}

#[test]
fn bounds_carry_no_uncertainty() {
    let point = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));
    let result = to_interval(&point, None).unwrap();

    assert_eq!(result.lower.uncertainty(), None);
    assert_eq!(result.upper.uncertainty(), None);
}

#[test]
fn before_true_when_widened_intervals_are_separated() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));
    let b = time_point("2027-04-20-12-10-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));

    assert_eq!(a.before(&b).unwrap(), TruthValue::True);
    assert_eq!(a.after(&b).unwrap(), TruthValue::False);
}

#[test]
fn before_unknown_when_widened_intervals_overlap() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(600));
    let b = time_point("2027-04-20-12-05-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(600));

    assert_eq!(a.before(&b).unwrap(), TruthValue::Unknown);
    assert_eq!(a.after(&b).unwrap(), TruthValue::Unknown);
}

#[test]
fn after_true_when_widened_intervals_are_separated() {
    let a = time_point("2027-04-20-12-10-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));
    let b = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));

    assert_eq!(a.after(&b).unwrap(), TruthValue::True);
}

#[test]
fn equals_true_when_widened_intervals_coincide() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(600));
    let b = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(600));

    assert_eq!(a.equals(&b).unwrap(), TruthValue::True);
}

#[test]
fn equals_unknown_when_widened_intervals_overlap() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(300));
    let b = time_point("2027-04-20-12-01-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(300));

    assert_eq!(a.equals(&b).unwrap(), TruthValue::Unknown);
}

#[test]
fn equals_false_when_widened_intervals_are_disjoint() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));
    let b = time_point("2027-04-20-12-10-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));

    assert_eq!(a.equals(&b).unwrap(), TruthValue::False);
}

#[test]
fn contains_unknown_when_partial_overlap() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(300));
    let b = time_point("2027-04-20-12-01-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(300));

    assert_eq!(a.contains(&b).unwrap(), TruthValue::Unknown);
}

#[test]
fn contains_true_when_widened_interval_subsets() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(600));
    let b = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(60));

    assert_eq!(a.contains(&b).unwrap(), TruthValue::True);
}

#[test]
fn overlaps_true_when_widened_intervals_share_points() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(300));
    let b = time_point("2027-04-20-12-01-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(300));

    assert_eq!(a.overlaps(&b).unwrap(), TruthValue::True);
}

#[test]
fn overlaps_false_when_widened_intervals_are_disjoint() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));
    let b = time_point("2027-04-20-12-10-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));

    assert_eq!(a.overlaps(&b).unwrap(), TruthValue::False);
}

#[test]
fn uncertainty_affects_relations_through_interval() {
    let nominal_a = time_point("2027-04-20-12-00-00").unwrap();
    let nominal_b = time_point("2027-04-20-12-00-05").unwrap();

    let ia = to_interval(&nominal_a, None).unwrap();
    let ib = to_interval(&nominal_b, None).unwrap();
    assert_eq!(ia.before(&ib), TruthValue::True);

    let ua = nominal_a.with_uncertainty(Uncertainty::from_seconds(10));
    let ub = nominal_b.with_uncertainty(Uncertainty::from_seconds(10));
    assert_eq!(ua.before(&ub).unwrap(), TruthValue::Unknown);
}

#[test]
fn explicit_interval_ignores_uncertainty() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));
    let b = time_point("2027-04-20-12-00-05").unwrap();

    let explicit = interval(&a, &b).unwrap();

    assert_eq!(explicit.lower, time_point("2027-04-20-12-00-00").unwrap());
    assert_eq!(explicit.upper, time_point("2027-04-20-12-00-05").unwrap());
}

#[test]
fn time_point_relations_with_same_uncertainty_are_consistent() {
    let a = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));
    let b = time_point("2027-04-20-12-00-05")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(1));

    assert_eq!(a.before(&b).unwrap(), TruthValue::True);
    assert_eq!(a.equals(&b).unwrap(), TruthValue::False);
}

#[test]
fn add_seconds_preserves_uncertainty() {
    let t = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));

    let result = t.add_seconds(60);

    assert_eq!(
        result,
        time_point("2027-04-20-12-01-00")
            .unwrap()
            .with_uncertainty(Uncertainty::from_seconds(10))
    );
}

#[test]
fn sub_seconds_preserves_uncertainty() {
    let t = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));

    let result = t.sub_seconds(60);

    assert_eq!(
        result,
        time_point("2027-04-20-11-59-00")
            .unwrap()
            .with_uncertainty(Uncertainty::from_seconds(10))
    );
}

#[test]
fn add_duration_preserves_uncertainty() {
    let t = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));

    let result = t.add_duration(date_time::duration::Duration::from_seconds(90));

    assert_eq!(
        result,
        time_point("2027-04-20-12-01-30")
            .unwrap()
            .with_uncertainty(Uncertainty::from_seconds(10))
    );

    let negative = t.add_duration(date_time::duration::Duration::from_seconds(-90));

    assert_eq!(
        negative,
        time_point("2027-04-20-11-58-30")
            .unwrap()
            .with_uncertainty(Uncertainty::from_seconds(10))
    );
}

#[test]
fn add_period_preserves_uncertainty() {
    let t = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(3600));

    let result = t.add_period(date_time::period::Period::from_months(2));

    assert_eq!(
        result,
        time_point("2027-06-20-12-00-00")
            .unwrap()
            .with_uncertainty(Uncertainty::from_seconds(3600))
    );

    let backwards = t.sub_period(date_time::period::Period::from_months(2));

    assert_eq!(
        backwards,
        time_point("2027-02-20-12-00-00")
            .unwrap()
            .with_uncertainty(Uncertainty::from_seconds(3600))
    );
}

#[test]
fn add_seconds_fast_preserves_uncertainty() {
    let t = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));

    let result = t.add_seconds_fast(60).unwrap();

    assert_eq!(
        result,
        time_point("2027-04-20-12-01-00")
            .unwrap()
            .with_uncertainty(Uncertainty::from_seconds(10))
    );
}

#[test]
fn shifted_uncertain_point_has_shifted_interval() {
    let t = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10));
    let shifted = t.add_duration(date_time::duration::Duration::from_seconds(5));

    let shifted_interval = to_interval(&shifted, None).unwrap();

    assert_eq!(
        shifted_interval.lower,
        time_point("2027-04-20-11-59-55").unwrap()
    );
    assert_eq!(
        shifted_interval.upper,
        time_point("2027-04-20-12-00-15").unwrap()
    );
}
