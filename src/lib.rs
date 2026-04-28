pub mod time_point;
pub mod precision;
pub mod interval;
pub mod util;
pub mod truth_values;

use crate::time_point::{TimePoint, time_point, encode_date, decode_year, decode_month, decode_day};
use crate::precision::Precision;
use crate::interval::{Interval, interval};
use crate::truth_values::TruthValue;


#[cfg(test)]
mod encode_decode_tests {
    use super::*;
    
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
}

#[cfg(test)] // done 
mod interval_tests {
    use crate::interval::to_interval;

    use super::*;
    
    #[test]
    fn constructs_day_to_day_interval() {
        let start = time_point("2027-04-10").unwrap();
        let end = time_point("2027-04-12").unwrap();

        let result = interval(&start, &end).unwrap();

        assert_eq!(
            result,
            Interval::new(TimePoint { year: 2027, month: 4, day: 10, precision: Precision::Day}, TimePoint { year: 2027, month: 4, day: 12, precision: Precision::Day  })
        );
    }

        #[test]
    fn to_interval_day_to_day() {
        let start = time_point("2027-04-10").unwrap();

        let result = to_interval(&start, None).unwrap();

        assert_eq!(
            result,
            Interval::new(TimePoint {
                    year: 2027,
                    month: 4,
                    day: 10,
                    precision: Precision::Day,
                }, TimePoint {
                    year: 2027,
                    month: 4,
                    day: 11,
                    precision: Precision::Day,
                })
        );
    }
}


#[cfg(test)] // done 
mod parse_tests {

    use super::*;
    #[test]
    fn parses_year_point() {
        let result = time_point("2027").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 1,
                day: 1,
                precision: Precision::Year,
            }
        );
    }

    #[test]
    fn parses_month_point() {
        let result = time_point("2027-11").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 11,
                day: 1,
                precision: Precision::Month,
            }
        );
    }

    #[test]
    fn parses_day_point() {
        let result = time_point("2027-04-20").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                precision: Precision::Day,
            }
        );
    }
    
    #[test]
    fn fails_on_invalid_month() {
        let result = time_point("2027-13-01");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_invalid_day() {
        let result = time_point("2027-04-40");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_too_many_parts() {
        let result = time_point("2027-04-10-12");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_empty_input() {
        let result = time_point("");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_non_numeric_year() {
        let result = time_point("abcd");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_non_numeric_month() {
        let result = time_point("2027-ab");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_non_numeric_day() {
        let result = time_point("2027-04-xx");
        assert!(result.is_err());
    }
}

#[cfg(test)] // Done 
mod normalization_tests {
    use crate::interval::{Interval, to_interval};

    use super::*;
    
    #[test]
    fn normalizes_year_to_interval() {
        let result = to_interval(&time_point("2027").unwrap(), None).unwrap();

        assert_eq!(
            result,
            Interval::new(TimePoint {
                    year: 2027,
                    month: 1,
                    day: 1,
                    precision: Precision::Year,
                }, TimePoint {
                    year: 2028,
                    month: 1,
                    day: 1,
                    precision: Precision::Year,
                })
        );
    }

    #[test]
    fn normalizes_month_to_interval() {
        let result = to_interval(&time_point("2027-04").unwrap(), None).unwrap();

        assert_eq!(
            result,
            Interval::new(
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 1,
                    precision: Precision::Month,
                },
                TimePoint {
                    year: 2027,
                    month: 5,
                    day: 1,
                    precision: Precision::Month,
                }
            )
        );
    }

    #[test]
    fn normalizes_day_to_interval() {
        let result = to_interval(&time_point("2027-04-20").unwrap(), None).unwrap();

        assert_eq!(
            result,
            Interval::new(
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    precision: Precision::Day,
                },
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 21,
                    precision: Precision::Day,
                }
            )
        );
    }

}

#[cfg(test)]
mod equals_tests {
    use super::*;

    #[test]
    fn equals_true_for_same_year() {
        let a = time_point("2027").unwrap();
        let b = time_point("2027").unwrap();

        assert_eq!(a.equals(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn equals_false_for_year_and_exact_day() {
        let a = time_point("2027").unwrap();
        let b = time_point("2027-01-01").unwrap();

        assert_eq!(a.equals(&b).unwrap(), TruthValue::False);
    }

    #[test]
    fn equals_true_for_same_month() {
        let a = time_point("2027-04").unwrap();
        let b = time_point("2027-04").unwrap();

        assert_eq!(a.equals(&b).unwrap(), TruthValue::True);
    }
}

#[cfg(test)]
mod before_tests {
    use super::*;

    #[test]
    fn before_true_for_adjacent_days() {
        let a = time_point("2027-01-01").unwrap();
        let b = time_point("2027-01-02").unwrap();

        assert_eq!(a.before(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn before_true_for_adjacent_months() {
        let a = time_point("2027-04").unwrap();
        let b = time_point("2027-05").unwrap();

        assert_eq!(a.before(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn before_false_for_year_and_month_inside_it() {
        let a = time_point("2027").unwrap();
        let b = time_point("2027-04").unwrap();

        assert_eq!(a.before(&b).unwrap(), TruthValue::False);
    }

}



#[cfg(test)]
mod after_tests {
    use super::*;

    #[test]
    fn after_true_for_adjacent_days() {
        let a = time_point("2027-01-02").unwrap();
        let b = time_point("2027-01-01").unwrap();

        assert_eq!(a.after(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn after_true_for_adjacent_months() {
        let a = time_point("2027-05").unwrap();
        let b = time_point("2027-04").unwrap();

        assert_eq!(a.after(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn after_false_for_month_and_year_containing_it() {
        let a = time_point("2027-04").unwrap();
        let b = time_point("2027").unwrap();

        assert_eq!(a.after(&b).unwrap(), TruthValue::False);
    }

    
}

#[cfg(test)]
mod contains_tests {
    use crate::interval::to_interval;

    use super::*;

    #[test]
    fn contains_true_for_adjacent_days() {
        let a = time_point("2027-01").unwrap();
        let b = time_point("2027-01-05").unwrap();

        let interval_a = to_interval(&a,None).unwrap();
        let interval_b = to_interval(&b,None).unwrap();

        assert_eq!(interval_a.contains(&interval_b), TruthValue::True);
    }

        #[test]
    fn contains_false_for_non_contained_month() {
        let a = time_point("2027-01").unwrap();
        let b = time_point("2027-02").unwrap();           

        let interval_a = to_interval(&a, None).unwrap();
        let interval_b = to_interval(&b, None).unwrap();

        assert_eq!(interval_a.contains(&interval_b), TruthValue::False);
    }

    #[test ]
    fn contains_true_for_same_month() {
        let a = time_point("2027-01").unwrap();
        let b = time_point("2027-01").unwrap();           

        let interval_a = to_interval(&a, None).unwrap();
        let interval_b = to_interval(&b, None).unwrap();

        assert_eq!(interval_a.contains(&interval_b), TruthValue::True);
    }
}

#[cfg(test)]
mod overlaps_tests {
    use crate::interval::to_interval;

    use super::*;

    #[test]
    fn overlaps_true_for_adjacent_days() {
        let a = time_point("2027-01").unwrap();
        let b = time_point("2027-01-05").unwrap();

        let interval_a = to_interval(&a, None).unwrap();
        let interval_b = to_interval(&b, None).unwrap();

        assert_eq!(interval_a.overlaps(&interval_b), TruthValue::True);
    }

    #[test] 
    fn overlaps_false_for_non_overlapping_month() {
        let a = time_point("2027-01").unwrap();
        let b = time_point("2027-02").unwrap();           

        let interval_a = to_interval(&a, None).unwrap();
        let interval_b = to_interval(&b, None).unwrap();

        assert_eq!(interval_a.overlaps(&interval_b), TruthValue::False);
    }

    #[test ]
    fn overlaps_true_for_same_month() {
        let a = time_point("2027-01").unwrap();
        let b = time_point("2027-01").unwrap();           

        let interval_a = to_interval(&a, None).unwrap();
        let interval_b = to_interval(&b, None).unwrap();

        assert_eq!(interval_a.overlaps(&interval_b), TruthValue::True);
    }
}
