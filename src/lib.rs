pub mod time_point;
pub mod precision;
pub mod interval;
pub mod util;
pub mod truth_values;
pub mod leap_second;
pub mod time_zone;

use crate::time_point::{TimePoint, time_point, encode_date, decode_year, decode_month, decode_day, encode_datetime};
use crate::precision::Precision;
use crate::interval::{Interval};
use crate::truth_values::TruthValue;
use crate::leap_second::{get_leap_seconds_data};
use crate::time_zone::TimeZone;

use pyo3::prelude::*;

#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct PyTimePoint {
    inner: TimePoint,
}

#[pymethods]
impl PyTimePoint {
    #[new]
    fn new(
        year: u32,
        month: Option<u32>,
        day: Option<u32>,
        hour: Option<u32>,
        minute: Option<u32>,
        second: Option<u32>,
        millisecond: Option<u32>,
    ) -> PyResult<Self> {
        let inner = TimePoint::new(year, month, day, hour, minute, second, millisecond)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(Self { inner })
    }

    #[staticmethod]
    fn parse(input: &str) -> PyResult<Self> {
        let inner = time_point(input)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(Self { inner })
    }

    #[staticmethod]
    fn now_utc() -> Self {
        Self {
            inner: TimePoint::now_utc(),
        }
    }

    fn add_minutes(&self, minutes: u64) -> Self {
        Self {
            inner: self.inner.add_minutes(minutes),
        }
    }

    fn add_hours(&self, hours: u64) -> Self {
        Self {
            inner: self.inner.add_hours(hours),
        }
    }

    fn sub_minutes(&self, minutes: u64) -> Self {
        Self {
            inner: self.inner.sub_minutes(minutes),
        }
    }

    fn sub_hours(&self, hours: u64) -> Self {
        Self {
            inner: self.inner.sub_hours(hours),
        }
    }

    fn add_seconds(&self, seconds: u64) -> Self {
        Self {
            inner: self.inner.add_seconds(seconds),
        }
    }

    fn sub_seconds(&self, seconds: u64) -> Self {
        Self {
            inner: self.inner.sub_seconds(seconds),
        }
    }

    fn before(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!("{:?}", self.inner.before(&other.inner)
            .map_err(pyo3::exceptions::PyValueError::new_err)?))
    }

    fn after(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!("{:?}", self.inner.after(&other.inner)
            .map_err(pyo3::exceptions::PyValueError::new_err)?))
    }

    fn contains(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!("{:?}", self.inner.contains(&other.inner)
            .map_err(pyo3::exceptions::PyValueError::new_err)?))
    }

    fn overlaps(&self, other: &PyTimePoint) -> PyResult<String> {
    Ok(format!(
        "{:?}",
        self.inner
            .overlaps(&other.inner)
            .map_err(pyo3::exceptions::PyValueError::new_err)?
    ))
}

    fn to_unix_timestamp(&self) -> PyResult<i64> {
        self.inner
            .to_unix_timestamp()
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    fn equals(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!(
            "{:?}",
            self.inner
                .equals(&other.inner)
                .map_err(pyo3::exceptions::PyValueError::new_err)?
        ))
    }

    fn __repr__(&self) -> String {
        format!(
            "TimePoint({:04}-{:02}-{:02}-{:02}-{:02}-{:02}-{:03}, precision={:?})",
            self.inner.year,
            self.inner.month,
            self.inner.day,
            self.inner.hour,
            self.inner.minute,
            self.inner.second,
            self.inner.millisecond,
            self.inner.precision,
        )
    }
}

#[pyfunction]
fn parse(input: &str) -> PyResult<PyTimePoint> {
    PyTimePoint::parse(input)
}

#[pymodule]
fn date_time(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTimePoint>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}

#[cfg(test)]
mod leap_second_tests {
    use super::*;

    #[test]
    fn get_leap_seconds_data_test() {
        println!("{:?}", get_leap_seconds_data());
    }


    #[test]
    fn leap_second_goes_from_59_to_60() {
        let t = time_point("2016-12-31-23-59-59").unwrap();
        let result = TimePoint::add_one_second(&t);

        assert_eq!(result.year, 2016);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 31);
        assert_eq!(result.hour, 23);
        assert_eq!(result.minute, 59);
        assert_eq!(result.second, 60);
    }

    #[test]
    fn leap_second_goes_from_60_to_next_day() {
        let t = time_point("2016-12-31-23-59-60").unwrap();
        let result = TimePoint::add_one_second(&t);

        assert_eq!(result.year, 2017);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);
        assert_eq!(result.hour, 0);
        assert_eq!(result.minute, 0);
        assert_eq!(result.second, 0);
    }

    #[test]
    fn normal_day_goes_from_59_to_next_day() {
        let t = time_point("2027-12-31-23-59-59").unwrap();
        let result = TimePoint::add_one_second(&t);

        assert_eq!(result.year, 2028);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);
        assert_eq!(result.hour, 0);
        assert_eq!(result.minute, 0);
        assert_eq!(result.second, 0);
    }

    #[test]
    fn add_seconds_includes_leap_second() {
        let t = time_point("2016-12-31-23-59-59").unwrap();
        let result = t.add_seconds(2);

        assert_eq!(result.year, 2017);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);
        assert_eq!(result.hour, 0);
        assert_eq!(result.minute, 0);
        assert_eq!(result.second, 0);
    }

    #[test]
    fn rejects_leap_second_on_non_leap_second_day() {
        let result = time_point("2027-12-31-23-59-60");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_leap_second_at_wrong_hour() {
        let result = time_point("2016-12-31-22-59-60");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_leap_second_at_wrong_minute() {
        let result = time_point("2016-12-31-23-58-60");
        assert!(result.is_err());
    }   

    #[test]
    fn sub_seconds_from_after_leap_second_returns_60() {
        let t = time_point("2017-01-01-00-00-00").unwrap();
        let result = t.sub_seconds(1);

        assert_eq!(result.year, 2016);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 31);
        assert_eq!(result.hour, 23);
        assert_eq!(result.minute, 59);
        assert_eq!(result.second, 60);
    }

    #[test]
    fn sub_seconds_from_leap_second_returns_59() {
        let t = time_point("2016-12-31-23-59-60").unwrap();
        let result = t.sub_seconds(1);

        assert_eq!(result.year, 2016);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 31);
        assert_eq!(result.hour, 23);
        assert_eq!(result.minute, 59);
        assert_eq!(result.second, 59);
    }

    #[test]
    fn sub_seconds_normal_day_rollover() {
        let t = time_point("2027-01-01-00-00-00").unwrap();
        let result = t.sub_seconds(1);

        assert_eq!(result.year, 2026);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 31);
        assert_eq!(result.hour, 23);
        assert_eq!(result.minute, 59);
        assert_eq!(result.second, 59);
    }

}




#[cfg(test)]
mod arithmetic_tests {
    use super::*;
    
    #[test]
    fn unix_timestamp_rejects_leap_second() {
        let t = time_point("2016-12-31-23-59-60").unwrap();

        assert!(t.to_unix_timestamp().is_err());
    }

    #[test]
    fn unix_roundtrip() {
        let t = time_point("2027-04-20-13-45-30").unwrap();

        let ts = t.to_unix_timestamp().unwrap();
        let back = TimePoint::from_unix_timestamp(ts);

        assert_eq!(t.year, back.year);
    }

    #[test]
    fn subtracts_one_second() {
        let t = time_point("2027-04-20-13-45-30").unwrap();
        let result = t.sub_seconds(1);

        assert_eq!(result.second, 29);
    }


    #[test]
    fn subtracts_second_with_minute_rollover() {
        let t = time_point("2027-04-20-13-45-00").unwrap();
        let result = t.sub_seconds(1);

        assert_eq!(result.minute, 44);
        assert_eq!(result.second, 59);
    }

    #[test]
    fn subtracts_second_with_day_rollover() {
        let t = time_point("2027-04-20-00-00-00").unwrap();
        let result = t.sub_seconds(1);

        assert_eq!(result.day, 19);
        assert_eq!(result.hour, 23);
        assert_eq!(result.minute, 59);
        assert_eq!(result.second, 59);
    }
    
    #[test]
    fn adds_one_second() {
        let t = time_point("2027-04-20-13-45-30").unwrap();
        let result = t.add_seconds(1);

        assert_eq!(result.second, 31);
    }

    #[test]
    fn second_rolls_to_next_minute() {
        let t = time_point("2027-04-20-13-45-59").unwrap();
        let result = t.add_seconds(1);

        assert_eq!(result.minute, 46);
        assert_eq!(result.second, 0);
    }

    #[test]
    fn minute_rolls_to_next_hour() {
        let t = time_point("2027-04-20-13-59-59").unwrap();
        let result = t.add_seconds(1);

        assert_eq!(result.hour, 14);
        assert_eq!(result.minute, 0);
        assert_eq!(result.second, 0);
    }

    #[test]
    fn hour_rolls_to_next_day() {
        let t = time_point("2027-04-20-23-59-59").unwrap();
        let result = t.add_seconds(1);

        assert_eq!(result.year, 2027);
        assert_eq!(result.month, 4);
        assert_eq!(result.day, 21);
        assert_eq!(result.hour, 0);
        assert_eq!(result.minute, 0);
        assert_eq!(result.second, 0);
    }

    #[test]
    fn day_rolls_to_next_month() {
        let t = time_point("2027-04-30-23-59-59").unwrap();
        let result = t.add_seconds(1);

        assert_eq!(result.year, 2027);
        assert_eq!(result.month, 5);
        assert_eq!(result.day, 1);
    }

    #[test]
    fn month_rolls_to_next_year() {
        let t = time_point("2027-12-31-23-59-59").unwrap();
        let result = t.add_seconds(1);

        assert_eq!(result.year, 2028);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);
    }

    #[test]
    fn adds_minutes() {
        let t = time_point("2027-04-20-13-45-30").unwrap();
        let result = t.add_minutes(2);

        assert_eq!(result.hour, 13);
        assert_eq!(result.minute, 47);
        assert_eq!(result.second, 30);
    }

    #[test]
    fn adds_hours() {
        let t = time_point("2027-04-20-13-45-30").unwrap();
        let result = t.add_hours(2);

        assert_eq!(result.hour, 15);
        assert_eq!(result.minute, 45);
        assert_eq!(result.second, 30);
    }

    #[test]
    fn arithmetic_preserves_precision() {
        let t = time_point("2027-04-20-13").unwrap();
        let result = t.add_seconds(30);

        assert_eq!(result.precision, Precision::Hour);
    }

    #[test]
    fn subtracting_from_after_leap_second_returns_60() {
        let t = time_point("2017-01-01-00-00-00").unwrap();
        let result = TimePoint::sub_one_second(&t);

        assert_eq!(result.year, 2016);
        assert_eq!(result.month, 12);
        assert_eq!(result.day, 31);
        assert_eq!(result.hour, 23);
        assert_eq!(result.minute, 59);
        assert_eq!(result.second, 60);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn now_utc_returns_second_precision() {
        let now = TimePoint::now_utc();
        
        println!("{:?}", now);
        assert_eq!(now.precision, Precision::Millisecond);
        assert!(now.month >= 1 && now.month <= 12);
        assert!(now.day >= 1 && now.day <= 31);
        assert!(now.hour <= 23);
        assert!(now.minute <= 59);
        assert!(now.second <= 59);
        assert!(now.millisecond <= 999);
    }

    #[test]
    fn constructs_year_precision_with_defaults() {
        let result = TimePoint::new(2027, None, None, None, None, None, None).unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                millisecond: 0,
                precision: Precision::Year,
                zone: TimeZone::UTC,
            }
        );
    }

    #[test]
    fn constructs_month_precision_with_defaults() {
        let result = TimePoint::new(2027, Some(4), None, None, None, None, None).unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                millisecond: 0,
                precision: Precision::Month,
                zone: TimeZone::UTC,
            }
        );
    }

    #[test]
    fn constructs_day_precision_with_defaults() {
        let result = TimePoint::new(2027, Some(4), Some(20), None, None, None, None).unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                hour: 0,
                minute: 0,
                second: 0,
                millisecond: 0,
                precision: Precision::Day,
                zone: TimeZone::UTC,
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
                millisecond: 0,
                precision: Precision::Hour,
                zone: TimeZone::UTC,
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
                millisecond: 0,
                precision: Precision::Minute,
                zone: TimeZone::UTC,
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
                second: 30,
                millisecond: 0,
                precision: Precision::Second,
                zone: TimeZone::UTC,
            }
        );
    }

    #[test]
    fn rejects_day_without_month() {
        let result = TimePoint::new(2027, None, Some(20), None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_hour_without_day() {
        let result = TimePoint::new(2027, Some(4), None, Some(13), None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_minute_without_hour() {
        let result = TimePoint::new(2027, Some(4), Some(20), None, Some(45), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_second_without_minute() {
        let result = TimePoint::new(2027, Some(4), Some(20), Some(13), None, Some(30), None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_month() {
        let result = TimePoint::new(2027, Some(13), None, None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_day() {
        let result = TimePoint::new(2027, Some(4), Some(40), None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_hour() {
        let result = TimePoint::new(2027, Some(4), Some(20), Some(24), None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_minute() {
        let result = TimePoint::new(2027, Some(4), Some(20), Some(13), Some(60), None, None);
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
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn supports_leap_year_date() {
        let result = TimePoint::new(2028, Some(2), Some(29), None, None, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_non_leap_year_date() {
        let result = TimePoint::new(2027, Some(2), Some(29), None, None, None, None);
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
        let a = encode_datetime(2027, 4, 10, 12, 0, 0, 0);
        let b = encode_datetime(2027, 4, 10, 13, 0, 0, 0);
        let c = encode_datetime(2027, 4, 10, 13, 1, 0, 0);
        let d = encode_datetime(2027, 4, 10, 13, 1, 1, 0);

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
                millisecond: 0,
                precision: Precision::Year,
                zone: TimeZone::UTC,
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
                millisecond: 0,
                precision: Precision::Month,
                zone: TimeZone::UTC,
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
                millisecond: 0,
                precision: Precision::Day,
                zone: TimeZone::UTC,
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
                millisecond: 0,
                precision: Precision::Hour,
                zone: TimeZone::UTC,
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
                millisecond: 0,
                precision: Precision::Minute,
                zone: TimeZone::UTC,
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
                millisecond: 0,
                precision: Precision::Second,
                zone: TimeZone::UTC,
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
        let result = time_point("2027-04-10-12-30-45-990-extra");
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
                    millisecond: 0,
                    precision: Precision::Year,
                    zone: TimeZone::UTC,
                },
                TimePoint {
                    year: 2028,
                    month: 1,
                    day: 1,
                    hour: 0,
                    minute: 0,
                    second: 0,
                    millisecond: 0,
                    precision: Precision::Year,
                    zone: TimeZone::UTC,
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
                    millisecond: 0,
                    precision: Precision::Hour,
                    zone: TimeZone::UTC,
                },
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 14,
                    minute: 0,
                    second: 0,
                    millisecond: 0,
                    precision: Precision::Hour,
                    zone: TimeZone::UTC,
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
                    millisecond: 0,
                    precision: Precision::Minute,
                    zone: TimeZone::UTC,
                },
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 13,
                    minute: 46,
                    second: 0,
                    millisecond: 0,
                    precision: Precision::Minute,
                    zone: TimeZone::UTC,
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
                    millisecond: 0,
                    precision: Precision::Second,
                    zone: TimeZone::UTC,
                },
                TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    hour: 13,
                    minute: 45,
                    second: 31,
                    millisecond: 0,
                    precision: Precision::Second,
                    zone: TimeZone::UTC,
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
}
