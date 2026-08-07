use date_time::interval::{AllenRelation, Interval, interval};
use date_time::time_point::TimePoint;
use date_time::truth_values::TruthValue;
use date_time::uncertainty::Uncertainty;

fn point(value: &str) -> TimePoint {
    date_time::time_point::time_point(value).unwrap()
}

fn iv(start: &str, end: &str) -> Interval {
    interval(&point(start), &point(end)).unwrap()
}

fn classify(a: &Interval, b: &Interval) -> AllenRelation {
    a.allen_relation(b).unwrap()
}

#[test]
fn before_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            &iv("2027-01-01-00-00-10", "2027-01-01-00-00-15")
        ),
        AllenRelation::Before
    );
}

#[test]
fn meets_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            &iv("2027-01-01-00-00-05", "2027-01-01-00-00-10")
        ),
        AllenRelation::Meets
    );
}

#[test]
fn overlaps_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-10"),
            &iv("2027-01-01-00-00-05", "2027-01-01-00-00-15")
        ),
        AllenRelation::Overlaps
    );
}

#[test]
fn contains_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            &iv("2027-01-01-00-00-05", "2027-01-01-00-00-10")
        ),
        AllenRelation::Contains
    );
}

#[test]
fn finished_by_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            &iv("2027-01-01-00-00-05", "2027-01-01-00-00-15")
        ),
        AllenRelation::FinishedBy
    );
}

#[test]
fn starts_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-15")
        ),
        AllenRelation::Starts
    );
}

#[test]
fn equal_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-05")
        ),
        AllenRelation::Equal
    );
}

#[test]
fn during_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-05", "2027-01-01-00-00-10"),
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-15")
        ),
        AllenRelation::During
    );
}

#[test]
fn started_by_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-05")
        ),
        AllenRelation::StartedBy
    );
}

#[test]
fn finishes_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-15")
        ),
        AllenRelation::Finishes
    );
}

#[test]
fn overlapped_by_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-10")
        ),
        AllenRelation::OverlappedBy
    );
}

#[test]
fn met_by_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-10", "2027-01-01-00-00-15"),
            &iv("2027-01-01-00-00-05", "2027-01-01-00-00-10")
        ),
        AllenRelation::MetBy
    );
}

#[test]
fn after_relation() {
    assert_eq!(
        classify(
            &iv("2027-01-01-00-00-10", "2027-01-01-00-00-15"),
            &iv("2027-01-01-00-00-00", "2027-01-01-00-00-05")
        ),
        AllenRelation::After
    );
}

#[test]
fn inverse_pairs_classify_as_inverses() {
    let pairs = [
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            iv("2027-01-01-00-00-10", "2027-01-01-00-00-15"),
            AllenRelation::Before,
            AllenRelation::After,
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-10"),
            AllenRelation::Meets,
            AllenRelation::MetBy,
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-10"),
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
            AllenRelation::Overlaps,
            AllenRelation::OverlappedBy,
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-10"),
            AllenRelation::Contains,
            AllenRelation::During,
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            AllenRelation::Starts,
            AllenRelation::StartedBy,
        ),
        (
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            AllenRelation::Finishes,
            AllenRelation::FinishedBy,
        ),
    ];

    for (a, b, forward, inverse) in pairs {
        assert_eq!(classify(&a, &b), forward);
        assert_eq!(classify(&b, &a), inverse);
    }
}

#[test]
fn equal_relation_is_symmetric() {
    let a = iv("2027-01-01-00-00-00", "2027-01-01-00-00-05");

    assert_eq!(classify(&a, &a), AllenRelation::Equal);
}

#[test]
fn classification_is_mutually_exclusive_and_exhaustive() {
    let intervals = [
        iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
        iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
        iv("2027-01-01-00-00-05", "2027-01-01-00-00-10"),
        iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
        iv("2027-01-01-00-00-10", "2027-01-01-00-00-15"),
    ];

    for a in &intervals {
        for b in &intervals {
            let r = classify(a, b);

            assert!(
                matches!(
                    r,
                    AllenRelation::Before
                        | AllenRelation::After
                        | AllenRelation::Meets
                        | AllenRelation::MetBy
                        | AllenRelation::Overlaps
                        | AllenRelation::OverlappedBy
                        | AllenRelation::Contains
                        | AllenRelation::During
                        | AllenRelation::Starts
                        | AllenRelation::StartedBy
                        | AllenRelation::Finishes
                        | AllenRelation::FinishedBy
                        | AllenRelation::Equal
                ),
                "pair must classify into exactly one of the 13 Allen relations, got {r:?}"
            );
        }
    }
}

fn truth_before(r: AllenRelation) -> TruthValue {
    match r {
        AllenRelation::Before | AllenRelation::Meets => TruthValue::True,
        AllenRelation::After | AllenRelation::MetBy => TruthValue::False,
        _ => TruthValue::Unknown,
    }
}

fn truth_after(r: AllenRelation) -> TruthValue {
    match r {
        AllenRelation::After | AllenRelation::MetBy => TruthValue::True,
        AllenRelation::Before | AllenRelation::Meets => TruthValue::False,
        _ => TruthValue::Unknown,
    }
}

fn truth_equals(r: AllenRelation) -> TruthValue {
    match r {
        AllenRelation::Equal => TruthValue::True,
        AllenRelation::Before
        | AllenRelation::Meets
        | AllenRelation::MetBy
        | AllenRelation::After => TruthValue::False,
        _ => TruthValue::Unknown,
    }
}

fn truth_contains(r: AllenRelation) -> TruthValue {
    match r {
        AllenRelation::Equal
        | AllenRelation::Contains
        | AllenRelation::StartedBy
        | AllenRelation::FinishedBy => TruthValue::True,
        AllenRelation::Before
        | AllenRelation::Meets
        | AllenRelation::MetBy
        | AllenRelation::After => TruthValue::False,
        _ => TruthValue::Unknown,
    }
}

fn truth_overlaps(r: AllenRelation) -> TruthValue {
    match r {
        AllenRelation::Overlaps
        | AllenRelation::OverlappedBy
        | AllenRelation::Contains
        | AllenRelation::During
        | AllenRelation::Starts
        | AllenRelation::StartedBy
        | AllenRelation::Finishes
        | AllenRelation::FinishedBy
        | AllenRelation::Equal => TruthValue::True,
        AllenRelation::Before
        | AllenRelation::Meets
        | AllenRelation::MetBy
        | AllenRelation::After => TruthValue::False,
    }
}

#[test]
fn three_valued_predicates_agree_with_allen_classification() {
    let pairs = [
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            iv("2027-01-01-00-00-10", "2027-01-01-00-00-15"),
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-10"),
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-10"),
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-10"),
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
        ),
        (
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-10"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
        ),
        (
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
        ),
        (
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-15"),
        ),
        (
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-10"),
        ),
        (
            iv("2027-01-01-00-00-10", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-05", "2027-01-01-00-00-10"),
        ),
        (
            iv("2027-01-01-00-00-10", "2027-01-01-00-00-15"),
            iv("2027-01-01-00-00-00", "2027-01-01-00-00-05"),
        ),
    ];

    for (a, b) in pairs {
        let r = classify(&a, &b);

        assert_eq!(a.before(&b), truth_before(r), "before {r:?}");
        assert_eq!(a.after(&b), truth_after(r), "after {r:?}");
        assert_eq!(a.equals(&b), truth_equals(r), "equals {r:?}");
        assert_eq!(a.contains(&b), truth_contains(r), "contains {r:?}");
        assert_eq!(a.overlaps(&b), truth_overlaps(r), "overlaps {r:?}");
    }
}

#[test]
fn time_point_level_precision_cases() {
    assert_eq!(
        point("2027-01-01-00-00-00")
            .allen_relation(&point("2027-01-01-00-00-10"))
            .unwrap(),
        AllenRelation::Before
    );
    assert_eq!(
        point("2027-01-01-00-00-00")
            .allen_relation(&point("2027-01-01-00-00-01"))
            .unwrap(),
        AllenRelation::Meets
    );
    assert_eq!(
        point("2027-04-20")
            .allen_relation(&point("2027-04-20-13"))
            .unwrap(),
        AllenRelation::Contains
    );
    assert_eq!(
        point("2027-04-20-13")
            .allen_relation(&point("2027-04-20-13-59"))
            .unwrap(),
        AllenRelation::FinishedBy
    );
    assert_eq!(
        point("2027-04-20-13-00")
            .allen_relation(&point("2027-04-20-13"))
            .unwrap(),
        AllenRelation::Starts
    );
    assert_eq!(
        point("2027-04-20-13-59")
            .allen_relation(&point("2027-04-20-13"))
            .unwrap(),
        AllenRelation::Finishes
    );
    assert_eq!(
        point("2027-04-20-13-30")
            .allen_relation(&point("2027-04-20-13-30"))
            .unwrap(),
        AllenRelation::Equal
    );
}

#[test]
fn allen_relation_with_uncertainty_expands_intervals() {
    let a = point("2027-04-20-12-00-00").with_uncertainty(Uncertainty::from_seconds(3600));
    let b = point("2027-04-20-14-00-00").with_uncertainty(Uncertainty::from_seconds(3600));

    assert_eq!(a.allen_relation(&b).unwrap(), AllenRelation::Meets);

    let c = point("2027-04-20-12-00-00").with_uncertainty(Uncertainty::from_seconds(600));
    let d = point("2027-04-20-12-05-00").with_uncertainty(Uncertainty::from_seconds(600));

    assert_eq!(c.allen_relation(&d).unwrap(), AllenRelation::Overlaps);
    assert_eq!(d.allen_relation(&c).unwrap(), AllenRelation::OverlappedBy);
}

#[test]
fn degenerate_interval_is_rejected() {
    let a = point("2027-04-20-13-45-30");
    let degenerate = interval(&a, &a).unwrap();

    assert_eq!(
        degenerate.allen_relation(&degenerate).unwrap_err(),
        "Interval is invalid, lower bound must be less than upper bound"
    );
}

#[test]
fn display_relation_names() {
    assert_eq!(AllenRelation::Before.to_string(), "before");
    assert_eq!(AllenRelation::After.to_string(), "after");
    assert_eq!(AllenRelation::Meets.to_string(), "meets");
    assert_eq!(AllenRelation::MetBy.to_string(), "met-by");
    assert_eq!(AllenRelation::Overlaps.to_string(), "overlaps");
    assert_eq!(AllenRelation::OverlappedBy.to_string(), "overlapped-by");
    assert_eq!(AllenRelation::Contains.to_string(), "contains");
    assert_eq!(AllenRelation::During.to_string(), "during");
    assert_eq!(AllenRelation::Starts.to_string(), "starts");
    assert_eq!(AllenRelation::StartedBy.to_string(), "started-by");
    assert_eq!(AllenRelation::Finishes.to_string(), "finishes");
    assert_eq!(AllenRelation::FinishedBy.to_string(), "finished-by");
    assert_eq!(AllenRelation::Equal.to_string(), "equal");
}

#[test]
fn precision_metadata_is_respected() {
    let a = point("2027-04-20-13");
    let b = point("2027-04-20-13-00");

    assert_eq!(a.allen_relation(&b).unwrap(), AllenRelation::StartedBy);
}
