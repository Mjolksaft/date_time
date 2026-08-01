use date_time::time_point::time_point;
use date_time::truth_values::TruthValue;

#[test]
fn before_true_for_seconds() {
    let a = time_point("2027-04-20-13-45-30").unwrap();
    let b = time_point("2027-04-20-13-45-31").unwrap();

    assert_eq!(a.before(&b).unwrap(), TruthValue::True);
}

#[test]
fn after_true_for_seconds() {
    let a = time_point("2027-04-20-13-45-31-0").unwrap();
    let b = time_point("2027-04-20-13-45-30-0").unwrap();

    assert_eq!(a.after(&b).unwrap(), TruthValue::True);
}

#[test]
fn equals_true_for_exact_second() {
    let a = time_point("2027-04-20-13-45-30").unwrap();
    let b = time_point("2027-04-20-13-45-30").unwrap();

    assert_eq!(a.equals(&b).unwrap(), TruthValue::True);
}
