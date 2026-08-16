//! PyO3 bindings exposing the core Rust temporal model to Python.

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
use crate::interval::{Interval, interval, to_interval};
use crate::period::Period;
use crate::time_point::{TimePoint, time_point};
use crate::time_zone::TimeZone;
use crate::uncertainty::Uncertainty;

use pyo3::prelude::*;

/// Python wrapper around [`TimeZone`]: UTC, TAI, Unix, or a fixed UTC offset.
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct PyTimeZone {
    inner: TimeZone,
}

#[pymethods]
impl PyTimeZone {
    #[staticmethod]
    fn fixed(hours: i8, minutes: i8) -> PyResult<Self> {
        let inner =
            TimeZone::fixed(hours, minutes).map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(Self { inner })
    }

    #[classattr]
    fn utc() -> Self {
        Self {
            inner: TimeZone::UTC,
        }
    }

    #[classattr]
    fn tai() -> Self {
        Self {
            inner: TimeZone::TAI,
        }
    }

    fn offset_label(&self) -> String {
        self.inner.offset_label()
    }

    fn __repr__(&self) -> String {
        format!("TimeZone({})", self.inner.offset_label())
    }
}

/// Python wrapper around [`Uncertainty`]: a symmetric offset in seconds.
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct PyUncertainty {
    inner: Uncertainty,
}

#[pymethods]
impl PyUncertainty {
    #[staticmethod]
    fn from_seconds(seconds: u64) -> Self {
        Self {
            inner: Uncertainty::from_seconds(seconds),
        }
    }

    fn seconds(&self) -> u64 {
        self.inner.seconds()
    }

    fn __repr__(&self) -> String {
        format!("Uncertainty({}s)", self.inner.seconds())
    }
}

/// Python wrapper around [`Duration`]: a fixed elapsed time with ms resolution.
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

/// Python wrapper around [`Period`]: a calendar-relative amount of time.
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

/// Python wrapper around [`Interval`]: a half-open `[lower, upper)` span.
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct PyInterval {
    inner: Interval,
}

#[pymethods]
impl PyInterval {
    #[staticmethod]
    fn interval(lower: &PyTimePoint, upper: &PyTimePoint) -> PyResult<Self> {
        let inner = interval(&lower.inner, &upper.inner)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(Self { inner })
    }

    #[staticmethod]
    #[pyo3(signature = (point, upper=None))]
    fn to_interval(point: &PyTimePoint, upper: Option<&PyTimePoint>) -> PyResult<Self> {
        let inner = to_interval(&point.inner, upper.map(|p| &p.inner))
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(Self { inner })
    }

    fn lower(&self) -> PyTimePoint {
        PyTimePoint {
            inner: self.inner.lower.clone(),
        }
    }

    fn upper(&self) -> PyTimePoint {
        PyTimePoint {
            inner: self.inner.upper.clone(),
        }
    }

    fn lower_key(&self) -> u64 {
        self.inner.lower_key()
    }

    fn upper_key(&self) -> u64 {
        self.inner.upper_key()
    }

    fn before(&self, other: &PyInterval) -> String {
        format!("{:?}", self.inner.before(&other.inner))
    }

    fn after(&self, other: &PyInterval) -> String {
        format!("{:?}", self.inner.after(&other.inner))
    }

    fn equals(&self, other: &PyInterval) -> String {
        format!("{:?}", self.inner.equals(&other.inner))
    }

    fn contains(&self, other: &PyInterval) -> String {
        format!("{:?}", self.inner.contains(&other.inner))
    }

    fn overlaps(&self, other: &PyInterval) -> String {
        format!("{:?}", self.inner.overlaps(&other.inner))
    }

    fn allen_relation(&self, other: &PyInterval) -> PyResult<String> {
        Ok(format!(
            "{}",
            self.inner
                .allen_relation(&other.inner)
                .map_err(pyo3::exceptions::PyValueError::new_err)?
        ))
    }

    fn __repr__(&self) -> String {
        let p = &self.inner.lower;
        let lower = format!(
            "{:04}-{:02}-{:02}-{:02}-{:02}-{:02}-{:03}",
            p.year, p.month, p.day, p.hour, p.minute, p.second, p.millisecond
        );
        let p = &self.inner.upper;
        let upper = format!(
            "{:04}-{:02}-{:02}-{:02}-{:02}-{:02}-{:03}",
            p.year, p.month, p.day, p.hour, p.minute, p.second, p.millisecond
        );

        format!("Interval([{lower}, {upper}))")
    }
}

/// Python wrapper around [`TimePoint`]: a wall clock with precision/zone/uncertainty.
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

    #[staticmethod]
    fn from_unix_timestamp(ts: i64) -> Self {
        Self {
            inner: TimePoint::from_unix_timestamp(ts),
        }
    }

    fn year(&self) -> u32 {
        self.inner.year
    }

    fn month(&self) -> u32 {
        self.inner.month
    }

    fn day(&self) -> u32 {
        self.inner.day
    }

    fn hour(&self) -> u32 {
        self.inner.hour
    }

    fn minute(&self) -> u32 {
        self.inner.minute
    }

    fn second(&self) -> u32 {
        self.inner.second
    }

    fn millisecond(&self) -> u32 {
        self.inner.millisecond
    }

    fn precision_label(&self) -> String {
        format!("{:?}", self.inner.precision)
    }

    fn uncertainty(&self) -> Option<PyUncertainty> {
        self.inner.uncertainty.map(|u| PyUncertainty { inner: u })
    }

    fn with_uncertainty(&self, uncertainty: &PyUncertainty) -> Self {
        Self {
            inner: self.inner.clone().with_uncertainty(uncertainty.inner),
        }
    }

    fn boundary_key(&self) -> u64 {
        self.inner.boundary_key()
    }

    fn add_seconds_fast(&self, seconds: u64) -> PyResult<Self> {
        let inner = self
            .inner
            .add_seconds_fast(seconds)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(Self { inner })
    }

    fn duration_until(&self, other: &PyTimePoint) -> PyResult<i64> {
        self.inner
            .duration_until(&other.inner)
            .map_err(pyo3::exceptions::PyValueError::new_err)
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

    fn zone_label(&self) -> String {
        self.inner.zone.offset_label()
    }

    fn utc_offset_seconds(&self) -> PyResult<i64> {
        self.inner
            .utc_offset_seconds()
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    fn with_zone(&self, zone: &PyTimeZone) -> Self {
        Self {
            inner: self.inner.with_zone(zone.inner),
        }
    }

    fn convert_to(&self, zone: &PyTimeZone) -> PyResult<Self> {
        let inner = self
            .inner
            .convert_to(zone.inner)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(Self { inner })
    }

    fn to_utc(&self) -> PyResult<Self> {
        let inner = self
            .inner
            .to_utc()
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(Self { inner })
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

/// Converts a UTC [`TimePoint`] to TAI, absorbing leap seconds into the offset.
#[pyfunction]
fn utc_to_tai(point: &PyTimePoint) -> PyResult<PyTimePoint> {
    let inner =
        crate::tai::utc_to_tai(&point.inner).map_err(pyo3::exceptions::PyValueError::new_err)?;

    Ok(PyTimePoint { inner })
}

/// Converts a TAI [`TimePoint`] back to UTC; leap seconds surface as `23:59:60`.
#[pyfunction]
fn tai_to_utc(point: &PyTimePoint) -> PyResult<PyTimePoint> {
    let inner =
        crate::tai::tai_to_utc(&point.inner).map_err(pyo3::exceptions::PyValueError::new_err)?;

    Ok(PyTimePoint { inner })
}

/// Returns the TAI-UTC offset (in seconds) valid at the given instant.
#[pyfunction]
fn tai_utc_offset(point: &PyTimePoint) -> PyResult<u32> {
    crate::tai::tai_utc_offset(&point.inner).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pymodule]
fn date_time(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTimePoint>()?;
    m.add_class::<PyDuration>()?;
    m.add_class::<PyPeriod>()?;
    m.add_class::<PyTimeZone>()?;
    m.add_class::<PyUncertainty>()?;
    m.add_class::<PyInterval>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(utc_to_tai, m)?)?;
    m.add_function(wrap_pyfunction!(tai_to_utc, m)?)?;
    m.add_function(wrap_pyfunction!(tai_utc_offset, m)?)?;
    Ok(())
}
