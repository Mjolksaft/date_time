use date_time::time_point::{decode_day, decode_month, decode_year, encode_date, encode_datetime};

#[test]
fn encodes_and_decodes_date_key() {
    let encoded = encode_date(2027, 4, 10);

    assert_eq!(decode_year(encoded), 2027);
    assert_eq!(decode_month(encoded), 4);
    assert_eq!(decode_day(encoded), 10);
}

#[test]
fn encoded_keys_preserve_order() {
    let a = encode_date(2027, 4, 10);
    let b = encode_date(2027, 4, 11);
    let c = encode_date(2027, 5, 1);
    let d = encode_date(2028, 1, 1);

    assert!(a < b);
    assert!(b < c);
    assert!(c < d);
}

#[test]
fn encoded_datetime_keys_preserve_order() {
    let a = encode_datetime(2027, 4, 10, 12, 0, 0, 0);
    let b = encode_datetime(2027, 4, 10, 13, 0, 0, 0);
    let c = encode_datetime(2027, 4, 10, 13, 1, 0, 0);
    let d = encode_datetime(2027, 4, 10, 13, 1, 1, 0);

    assert!(a < b);
    assert!(b < c);
    assert!(c < d);
}

#[test]
fn encoded_datetime_keys_hold_across_unit_boundaries() {
    assert!(encode_datetime(2027, 4, 10, 12, 0, 59, 0) < encode_datetime(2027, 4, 10, 12, 1, 0, 0));
    assert!(
        encode_datetime(2027, 4, 10, 12, 59, 59, 999) < encode_datetime(2027, 4, 10, 13, 0, 0, 0)
    );
    assert!(
        encode_datetime(2027, 4, 10, 23, 59, 59, 999) < encode_datetime(2027, 4, 11, 0, 0, 0, 0)
    );
    assert!(
        encode_datetime(2027, 4, 30, 23, 59, 59, 999) < encode_datetime(2027, 5, 1, 0, 0, 0, 0)
    );
    assert!(
        encode_datetime(2027, 12, 31, 23, 59, 59, 999) < encode_datetime(2028, 1, 1, 0, 0, 0, 0)
    );
}

#[test]
fn encoded_datetime_keys_handle_leap_second() {
    assert!(
        encode_datetime(2016, 12, 31, 23, 59, 59, 0) < encode_datetime(2016, 12, 31, 23, 59, 60, 0)
    );
    assert!(encode_datetime(2016, 12, 31, 23, 59, 60, 0) < encode_datetime(2017, 1, 1, 0, 0, 0, 0));
}
