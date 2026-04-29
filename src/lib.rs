pub mod time_point;
pub mod precision;
pub mod interval;
pub mod util;
pub mod truth_values;
pub mod leap_second;

use crate::time_point::{TimePoint, time_point, encode_date, decode_year, decode_month, decode_day, encode_datetime};
use crate::precision::Precision;
use crate::interval::{Interval};
use crate::truth_values::TruthValue;
use crate::leap_second::{get_leap_seconds_data};


#[cfg(test)]
mod leap_second_tests {
    use super::*;

    #[test]
    fn get_leap_seconds_data_test() {
        println!("{:?}", get_leap_seconds_data());
    }

}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn now_utc_returns_second_precision() {
        let now = TimePoint::now_utc();

        assert_eq!(now.precision, Precision::Second);
        assert!(now.month >= 1 && now.month <= 12);
        assert!(now.day >= 1 && now.day <= 31);
        assert!(now.hour <= 23);
        assert!(now.minute <= 59);
        assert!(now.second <= 59);
    }

    #[test]
    fn constructs_year_precision_with_defaults() {
        let result = TimePoint::new(2027, None, None, None, None, None).unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                precision: Precision::Year,
            }
        );
    }

    #[test]
    fn constructs_month_precision_with_defaults() {
        let result = TimePoint::new(2027, Some(4), None, None, None, None).unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                precision: Precision::Month,
            }
        );
    }

    #[test]
    fn constructs_day_precision_with_defaults() {
        let result = TimePoint::new(2027, Some(4), Some(20), None, None, None).unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 0,
                minute: 0,
                second: 0,
                precision: Precision::Day,
            }
        );
    }

    #[test]
    fn constructs_hour_precision_with_defaults() {
        let result = TimePoint::new(
            2027,
            Some(4),
            Some(20),
            Some(13),
            None,
            None,
        )
        .unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 13,
                minute: 0,
                second: 0,
                precision: Precision::Hour,
            }
        );
    }

    #[test]
    fn constructs_minute_precision_with_defaults() {
        let result = TimePoint::new(
            2027,
            Some(4),
            Some(20),
            Some(13),
            Some(45),
            None,
        )
        .unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 13,
                minute: 45,
                second: 0,
                precision: Precision::Minute,
            }
        );
    }

    #[test]
    fn constructs_second_precision() {
        let result = TimePoint::new(
            2027,
            Some(4),
            Some(20),
            Some(13),
            Some(45),
            Some(30),
        )
        .unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 13,
                minute: 45,
                second: 30,
                precision: Precision::Second,
            }
        );
    }

    #[test]
    fn rejects_day_without_month() {
        let result = TimePoint::new(2027, None, Some(20), None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_hour_without_day() {
        let result = TimePoint::new(2027, Some(4), None, Some(13), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_minute_without_hour() {
        let result = TimePoint::new(2027, Some(4), Some(20), None, Some(45), None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_second_without_minute() {
        let result = TimePoint::new(2027, Some(4), Some(20), Some(13), None, Some(30));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_month() {
        let result = TimePoint::new(2027, Some(13), None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_day() {
        let result = TimePoint::new(2027, Some(4), Some(40), None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_hour() {
        let result = TimePoint::new(2027, Some(4), Some(20), Some(24), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_minute() {
        let result = TimePoint::new(2027, Some(4), Some(20), Some(13), Some(60), None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_second() {
        let result = TimePoint::new(
            2027,
            Some(4),
            Some(20),
            Some(13),
            Some(45),
            Some(60),
        );

        assert!(result.is_err());
    }

    #[test]
    fn supports_leap_year_date() {
        let result = TimePoint::new(2028, Some(2), Some(29), None, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_non_leap_year_date() {
        let result = TimePoint::new(2027, Some(2), Some(29), None, None, None);
        assert!(result.is_err());
    }
}

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

    #[test]
    fn encoded_datetime_keys_preserve_order() {
        let a = encode_datetime(2027, 4, 10, 12, 0, 0);
        let b = encode_datetime(2027, 4, 10, 13, 0, 0);
        let c = encode_datetime(2027, 4, 10, 13, 1, 0);
        let d = encode_datetime(2027, 4, 10, 13, 1, 1);

        assert!(a < b);
        assert!(b < c);
        assert!(c < d);
    }
}

#[cfg(test)]
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
                hour: 0,
                minute: 0,
                second: 0,
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
                hour: 0,
                minute: 0,
                second: 0,
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
                hour: 0,
                minute: 0,
                second: 0,
                precision: Precision::Day,
            }
        );
    }

    #[test]
    fn parses_hour_point() {
        let result = time_point("2027-04-20-13").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 13,
                minute: 0,
                second: 0,
                precision: Precision::Hour,
            }
        );
    }

    #[test]
    fn parses_minute_point() {
        let result = time_point("2027-04-20-13-45").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 13,
                minute: 45,
                second: 0,
                precision: Precision::Minute,
            }
        );
    }

    #[test]
    fn parses_second_point() {
        let result = time_point("2027-04-20-13-45-30").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 13,
                minute: 45,
                second: 30,
                precision: Precision::Second,
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
    fn fails_on_invalid_hour() {
        let result = time_point("2027-04-20-24");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_invalid_minute() {
        let result = time_point("2027-04-20-13-60");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_invalid_second() {
        let result = time_point("2027-04-20-13-45-60");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_too_many_parts() {
        let result = time_point("2027-04-10-12-30-45-99");
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
}

#[cfg(test)]
mod normalization_tests {
    use super::*;
    use crate::interval::to_interval;

    #[test]
    fn normalizes_year_to_interval() {
        let result = to_interval(&time_point("2027").unwrap(), None).unwrap();

        assert_eq!(
            result,
            Interval::new(
                TimePoint {
                    year: 2027,
                    month: 1,
                    day: 1,
                    hour: 0,
                    minute: 0,
                    second: 0,
                    precision: Precision::Year,
                },
                TimePoint {
                    year: 2028,
                    month: 1,
                    day: 1,
                    hour: 0,
                    minute: 0,
                    second: 0,
                    precision: Precision::Year,
                }
            )
        );
    }

    #[test]
    fn normalizes_hour_to_interval() {
        let result = to_interval(&time_point("2027-04-20-13").unwrap(), None).unwrap();

        assert_eq!(
            result,
            Interval::new(
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 13,
                    minute: 0,
                    second: 0,
                    precision: Precision::Hour,
                },
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 14,
                    minute: 0,
                    second: 0,
                    precision: Precision::Hour,
                }
            )
        );
    }

    #[test]
    fn normalizes_minute_to_interval() {
        let result = to_interval(&time_point("2027-04-20-13-45").unwrap(), None).unwrap();

        assert_eq!(
            result,
            Interval::new(
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 13,
                    minute: 45,
                    second: 0,
                    precision: Precision::Minute,
                },
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 13,
                    minute: 46,
                    second: 0,
                    precision: Precision::Minute,
                }
            )
        );
    }

    #[test]
    fn normalizes_second_to_interval() {
        let result = to_interval(&time_point("2027-04-20-13-45-30").unwrap(), None).unwrap();

        assert_eq!(
            result,
            Interval::new(
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 13,
                    minute: 45,
                    second: 30,
                    precision: Precision::Second,
                },
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 13,
                    minute: 45,
                    second: 31,
                    precision: Precision::Second,
                }
            )
        );
    }
}

#[cfg(test)]
mod rollover_tests {
    use super::*;
    use crate::interval::to_interval;

    #[test]
    fn hour_rollover_to_next_day() {
        let result = to_interval(&time_point("2027-04-20-23").unwrap(), None).unwrap();

        assert_eq!(result.upper.hour, 0);
        assert_eq!(result.upper.day, 21);
    }

    #[test]
    fn minute_rollover_to_next_hour() {
        let result = to_interval(&time_point("2027-04-20-13-59").unwrap(), None).unwrap();

        assert_eq!(result.upper.hour, 14);
        assert_eq!(result.upper.minute, 0);
    }

    #[test]
    fn second_rollover_to_next_minute() {
        let result = to_interval(&time_point("2027-04-20-13-45-59").unwrap(), None).unwrap();

        assert_eq!(result.upper.minute, 46);
        assert_eq!(result.upper.second, 0);
    }
}

#[cfg(test)]
mod comparison_tests {
    use super::*;

    #[test]
    fn before_true_for_seconds() {
        let a = time_point("2027-04-20-13-45-30").unwrap();
        let b = time_point("2027-04-20-13-45-31").unwrap();

        assert_eq!(a.before(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn after_true_for_seconds() {
        let a = time_point("2027-04-20-13-45-31").unwrap();
        let b = time_point("2027-04-20-13-45-30").unwrap();

        assert_eq!(a.after(&b).unwrap(), TruthValue::True);
    }

    #[test]
    fn equals_true_for_exact_second() {
        let a = time_point("2027-04-20-13-45-30").unwrap();
        let b = time_point("2027-04-20-13-45-30").unwrap();

        assert_eq!(a.equals(&b).unwrap(), TruthValue::True);
    }
}