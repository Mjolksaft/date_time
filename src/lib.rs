pub mod duration;
pub mod interval;
pub mod leap_second;
pub mod period;
pub mod precision;
pub mod tai;
pub mod time_point;
pub mod time_zone;
pub mod truth_values;
pub mod uncertainty;
pub mod unix;
pub mod util;

use crate::duration::Duration;
use crate::period::Period;
use crate::time_point::{TimePoint, time_point};

use pyo3::prelude::*;

#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct PyDuration {
    inner: Duration,
}

#[pymethods]
impl PyDuration {
    #[staticmethod]
    fn from_seconds(seconds: i64) -> Self {
        Self {
            inner: Duration::from_seconds(seconds),
        }
    }

    #[staticmethod]
    fn from_milliseconds(milliseconds: i64) -> Self {
        Self {
            inner: Duration::from_milliseconds(milliseconds),
        }
    }

    fn milliseconds(&self) -> i64 {
        self.inner.milliseconds()
    }

    fn seconds(&self) -> i64 {
        self.inner.seconds()
    }

    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    fn is_negative(&self) -> bool {
        self.inner.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.inner.is_positive()
    }

    fn __repr__(&self) -> String {
        format!("Duration({})", self.inner)
    }
}

#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct PyPeriod {
    inner: Period,
}

#[pymethods]
impl PyPeriod {
    #[new]
    #[pyo3(signature = (years=0, months=0, days=0, hours=0, minutes=0, seconds=0, milliseconds=0))]
    fn new(
        years: i64,
        months: i64,
        days: i64,
        hours: i64,
        minutes: i64,
        seconds: i64,
        milliseconds: i64,
    ) -> Self {
        Self {
            inner: Period::new(years, months, days, hours, minutes, seconds, milliseconds),
        }
    }

    #[staticmethod]
    fn from_years(years: i64) -> Self {
        Self {
            inner: Period::from_years(years),
        }
    }

    #[staticmethod]
    fn from_months(months: i64) -> Self {
        Self {
            inner: Period::from_months(months),
        }
    }

    #[staticmethod]
    fn from_days(days: i64) -> Self {
        Self {
            inner: Period::from_days(days),
        }
    }

    fn years(&self) -> i64 {
        self.inner.years()
    }

    fn months(&self) -> i64 {
        self.inner.months()
    }

    fn days(&self) -> i64 {
        self.inner.days()
    }

    fn hours(&self) -> i64 {
        self.inner.hours()
    }

    fn minutes(&self) -> i64 {
        self.inner.minutes()
    }

    fn seconds(&self) -> i64 {
        self.inner.seconds()
    }

    fn milliseconds(&self) -> i64 {
        self.inner.milliseconds()
    }

    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    fn is_negative(&self) -> bool {
        self.inner.is_negative()
    }

    fn __repr__(&self) -> String {
        format!("Period({})", self.inner)
    }
}

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
        let inner = time_point(input).map_err(pyo3::exceptions::PyValueError::new_err)?;

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

    fn add_duration(&self, duration: &PyDuration) -> Self {
        Self {
            inner: self.inner.add_duration(duration.inner),
        }
    }

    fn sub_duration(&self, duration: &PyDuration) -> Self {
        Self {
            inner: self.inner.sub_duration(duration.inner),
        }
    }

    fn duration_since(&self, earlier: &PyTimePoint) -> PyDuration {
        PyDuration {
            inner: self.inner.duration_since(&earlier.inner),
        }
    }

    fn add_period(&self, period: &PyPeriod) -> Self {
        Self {
            inner: self.inner.add_period(period.inner),
        }
    }

    fn sub_period(&self, period: &PyPeriod) -> Self {
        Self {
            inner: self.inner.sub_period(period.inner),
        }
    }

    fn before(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!(
            "{:?}",
            self.inner
                .before(&other.inner)
                .map_err(pyo3::exceptions::PyValueError::new_err)?
        ))
    }

    fn after(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!(
            "{:?}",
            self.inner
                .after(&other.inner)
                .map_err(pyo3::exceptions::PyValueError::new_err)?
        ))
    }

    fn contains(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!(
            "{:?}",
            self.inner
                .contains(&other.inner)
                .map_err(pyo3::exceptions::PyValueError::new_err)?
        ))
    }

    fn overlaps(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!(
            "{:?}",
            self.inner
                .overlaps(&other.inner)
                .map_err(pyo3::exceptions::PyValueError::new_err)?
        ))
    }

    fn allen_relation(&self, other: &PyTimePoint) -> PyResult<String> {
        Ok(format!(
            "{}",
            self.inner
                .allen_relation(&other.inner)
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
    m.add_class::<PyDuration>()?;
    m.add_class::<PyPeriod>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
