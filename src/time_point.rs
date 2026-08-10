use std::cmp::Ordering;
use std::ops::{Add, Sub};
use time::OffsetDateTime;

use crate::duration::Duration;
use crate::interval::{AllenRelation, to_interval};
use crate::leap_second::is_leap_second;
use crate::period::Period;
use crate::precision::Precision;
use crate::time_zone::TimeZone;
use crate::truth_values::TruthValue;
use crate::uncertainty::Uncertainty;
use crate::util::{days_in_month, valid_date};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimePoint {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
    pub precision: Precision,
    pub zone: TimeZone,
    pub uncertainty: Option<Uncertainty>,
}

fn start_of(year: u32, month: u32, day: u32, precision: Precision, zone: TimeZone) -> TimePoint {
    TimePoint {
        year,
        month,
        day,
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
        precision,
        zone,
        uncertainty: None,
    }
}

impl TimePoint {
    pub fn new(
        year: u32,
        month: Option<u32>,
        day: Option<u32>,
        hour: Option<u32>,
        minute: Option<u32>,
        second: Option<u32>,
        millisecond: Option<u32>,
    ) -> Result<Self, String> {
        let precision = match (month, day, hour, minute, second, millisecond) {
            (None, None, None, None, None, None) => Precision::Year,
            (Some(_), None, None, None, None, None) => Precision::Month,
            (Some(_), Some(_), None, None, None, None) => Precision::Day,
            (Some(_), Some(_), Some(_), None, None, None) => Precision::Hour,
            (Some(_), Some(_), Some(_), Some(_), None, None) => Precision::Minute,
            (Some(_), Some(_), Some(_), Some(_), Some(_), None) => Precision::Second,
            (Some(_), Some(_), Some(_), Some(_), Some(_), Some(_)) => Precision::Millisecond,
            _ => return Err(String::from("Invalid missing fields order")),
        };

        let month = month.unwrap_or(1);
        let day = day.unwrap_or(1);
        let hour = hour.unwrap_or(0);
        let minute = minute.unwrap_or(0);
        let second = second.unwrap_or(0);
        let millisecond = millisecond.unwrap_or(0);
        valid_date(
            year,
            Some(month),
            Some(day),
            Some(hour),
            Some(minute),
            Some(second),
            Some(millisecond),
        )?;
        if second == 60 {
            if hour != 23 || minute != 59 || !is_leap_second(year, month, day) {
                return Err(String::from("Invalid leap second"));
            }
        }

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
            precision,
            zone: TimeZone::UTC,
            uncertainty: None,
        })
    }

    pub fn now_utc() -> Self {
        let now = OffsetDateTime::now_utc();

        TimePoint {
            year: now.year() as u32,
            month: now.month() as u32,
            day: now.day() as u32,
            hour: now.hour() as u32,
            minute: now.minute() as u32,
            second: now.second() as u32,
            millisecond: now.nanosecond() / 1_000_000,
            precision: Precision::Millisecond,
            zone: TimeZone::UTC,
            uncertainty: None,
        }
    }

    pub fn with_uncertainty(mut self, uncertainty: Uncertainty) -> Self {
        self.uncertainty = Some(uncertainty);
        self
    }

    pub fn uncertainty(&self) -> Option<Uncertainty> {
        self.uncertainty
    }

    /// Attaches a zone to this point without converting its wall clock.
    pub fn with_zone(&self, zone: TimeZone) -> Self {
        TimePoint {
            zone,
            ..self.clone()
        }
    }

    /// The UTC offset in seconds for the zone this point is expressed in.
    pub fn utc_offset_seconds(&self) -> Result<i64, String> {
        self.zone.utc_offset_seconds()
    }

    /// Interprets this point's wall clock together with its zone offset as a
    /// UTC wall clock. Errors for TAI, which is not a fixed-offset zone.
    pub fn as_utc(&self) -> Result<TimePoint, String> {
        let offset = self.utc_offset_seconds()?;
        let uncertainty = self.uncertainty;

        let mut utc = if offset > 0 {
            self.sub_seconds(offset as u64)
        } else {
            self.add_seconds((-offset) as u64)
        };

        utc.zone = TimeZone::UTC;
        utc.uncertainty = uncertainty;
        Ok(utc)
    }

    /// Converts this point to the equivalent wall clock in another zone.
    ///
    /// Supports UTC, `Unix`, and fixed-offset zones directly. Converting to or
    /// from TAI routes through the leap-second-aware TAI conversion.
    pub fn convert_to(&self, zone: TimeZone) -> Result<TimePoint, String> {
        if self.zone == zone {
            return Ok(self.clone());
        }

        let uncertainty = self.uncertainty;

        let utc = match self.zone {
            TimeZone::TAI => crate::tai::tai_to_utc(self)?,
            _ => self.as_utc()?,
        };

        let mut result = match zone {
            TimeZone::TAI => crate::tai::utc_to_tai(&utc)?,
            TimeZone::UTC | TimeZone::Unix => utc.with_zone(zone),
            TimeZone::Fixed { hours, minutes } => {
                let offset = i64::from(hours) * 3600 + i64::from(minutes) * 60;
                let mut local = if offset > 0 {
                    utc.add_seconds(offset as u64)
                } else {
                    utc.sub_seconds((-offset) as u64)
                };
                local.zone = zone;
                local
            }
        };

        result.uncertainty = uncertainty;
        Ok(result)
    }

    /// Convenience conversion to UTC.
    pub fn to_utc(&self) -> Result<TimePoint, String> {
        self.convert_to(TimeZone::UTC)
    }
}

pub fn time_point(input: &str) -> Result<TimePoint, String> {
    if input.is_empty() {
        return Err(String::from("No args"));
    }

    let parsed_date: Vec<u32> = parse_date_time_point(input)?;

    match parsed_date.len() {
        1 => TimePoint::new(parsed_date[0], None, None, None, None, None, None),

        2 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            None,
            None,
            None,
            None,
            None,
        ),

        3 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            Some(parsed_date[2]),
            None,
            None,
            None,
            None,
        ),

        4 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            Some(parsed_date[2]),
            Some(parsed_date[3]),
            None,
            None,
            None,
        ),

        5 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            Some(parsed_date[2]),
            Some(parsed_date[3]),
            Some(parsed_date[4]),
            None,
            None,
        ),

        6 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            Some(parsed_date[2]),
            Some(parsed_date[3]),
            Some(parsed_date[4]),
            Some(parsed_date[5]),
            None,
        ),

        7 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            Some(parsed_date[2]),
            Some(parsed_date[3]),
            Some(parsed_date[4]),
            Some(parsed_date[5]),
            Some(parsed_date[6]),
        ),

        _ => Err(String::from("Invalid date format")),
    }
}

pub fn parse_date_time_point(input: &str) -> Result<Vec<u32>, String> {
    let parts: Vec<&str> = input.split("-").collect();

    parts
        .iter()
        .map(|x| {
            x.parse::<u32>()
                .map_err(|_| String::from("Invalid number format"))
        })
        .collect()
}

impl Ord for TimePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
        )
            .cmp(&(
                other.year,
                other.month,
                other.day,
                other.hour,
                other.minute,
                other.second,
                other.millisecond,
            ))
    }
}

impl PartialOrd for TimePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl TimePoint {
    pub fn to_unix_timestamp(&self) -> Result<i64, String> {
        crate::unix::to_unix_timestamp(self)
    }

    pub fn from_unix_timestamp(ts: i64) -> Self {
        crate::unix::from_unix_timestamp(ts)
    }

    pub fn boundary_key(&self) -> u64 {
        encode_datetime(
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
        )
    }

    pub fn add_seconds_fast(&self, seconds: u64) -> Result<Self, String> {
        let ts = self.to_unix_timestamp()?;
        let mut result = TimePoint::from_unix_timestamp(ts + seconds as i64);

        result.precision = self.precision.clone();
        result.zone = self.zone.clone();
        result.uncertainty = self.uncertainty;

        Ok(result)
    }

    pub fn add_seconds(&self, seconds: u64) -> Self {
        let uncertainty = self.uncertainty;
        let mut result = self.clone();

        for _ in 0..seconds {
            result = Self::add_one_second(&result);
        }

        result.uncertainty = uncertainty;
        result
    }

    pub fn add_minutes(&self, minutes: u64) -> Self {
        self.add_seconds(minutes * 60)
    }

    pub fn add_hours(&self, hours: u64) -> Self {
        self.add_seconds(hours * 60 * 60)
    }

    pub fn add_one_millisecond(t: &TimePoint) -> TimePoint {
        if t.millisecond == 999 {
            let next_second = Self::add_one_second(t);

            return TimePoint {
                millisecond: 0,
                precision: t.precision.clone(),
                zone: t.zone.clone(),
                ..next_second
            };
        }

        TimePoint {
            year: t.year,
            month: t.month,
            day: t.day,
            hour: t.hour,
            minute: t.minute,
            second: t.second,
            millisecond: t.millisecond + 1,
            precision: t.precision.clone(),
            zone: t.zone.clone(),
            uncertainty: None,
        }
    }

    pub fn add_one_second(t: &TimePoint) -> TimePoint {
        if t.second == 59 && is_leap_second(t.year, t.month, t.day) {
            return TimePoint {
                year: t.year,
                month: t.month,
                day: t.day,
                hour: t.hour,
                minute: t.minute,
                second: 60,
                millisecond: 0,
                precision: t.precision.clone(),
                zone: t.zone.clone(),
                uncertainty: None,
            };
        }

        if t.second == 60 || t.second == 59 {
            let next_minute = Self::add_one_minute(t);

            return TimePoint {
                millisecond: 0,
                precision: t.precision.clone(),
                zone: t.zone.clone(),
                ..next_minute
            };
        }

        TimePoint {
            year: t.year,
            month: t.month,
            day: t.day,
            hour: t.hour,
            minute: t.minute,
            second: t.second + 1,
            millisecond: 0,
            precision: t.precision.clone(),
            zone: t.zone.clone(),
            uncertainty: None,
        }
    }

    pub fn add_one_minute(t: &TimePoint) -> TimePoint {
        if t.minute == 59 {
            let next_hour = Self::add_one_hour(t);

            TimePoint {
                precision: t.precision.clone(),
                ..next_hour
            }
        } else {
            TimePoint {
                year: t.year,
                month: t.month,
                day: t.day,
                hour: t.hour,
                minute: t.minute + 1,
                second: 0,
                millisecond: 0,
                precision: t.precision.clone(),
                zone: t.zone.clone(),
                uncertainty: None,
            }
        }
    }

    pub fn add_one_hour(t: &TimePoint) -> TimePoint {
        if t.hour == 23 {
            let next_day = Self::add_one_day(t);

            TimePoint {
                precision: t.precision.clone(),
                ..next_day
            }
        } else {
            TimePoint {
                year: t.year,
                month: t.month,
                day: t.day,
                hour: t.hour + 1,
                minute: 0,
                second: 0,
                millisecond: 0,
                precision: t.precision.clone(),
                zone: t.zone.clone(),
                uncertainty: None,
            }
        }
    }

    pub fn add_one_day(t: &TimePoint) -> TimePoint {
        let days = days_in_month(t.year, t.month);
        let zone = t.zone.clone();

        if t.day == days {
            if t.month == 12 {
                start_of(t.year + 1, 1, 1, t.precision.clone(), zone)
            } else {
                start_of(t.year, t.month + 1, 1, t.precision.clone(), zone)
            }
        } else {
            start_of(t.year, t.month, t.day + 1, t.precision.clone(), zone)
        }
    }

    pub fn add_one_month(t: &TimePoint) -> TimePoint {
        if t.month == 12 {
            start_of(t.year + 1, 1, 1, t.precision.clone(), t.zone.clone())
        } else {
            start_of(t.year, t.month + 1, 1, t.precision.clone(), t.zone.clone())
        }
    }

    pub fn add_one_year(t: &TimePoint) -> TimePoint {
        start_of(t.year + 1, 1, 1, t.precision.clone(), t.zone.clone())
    }

    pub fn sub_seconds(&self, seconds: u64) -> Self {
        let uncertainty = self.uncertainty;
        let mut result = self.clone();

        for _ in 0..seconds {
            result = Self::sub_one_second(&result);
        }

        result.uncertainty = uncertainty;
        result
    }

    pub fn sub_minutes(&self, minutes: u64) -> Self {
        self.sub_seconds(minutes * 60)
    }

    pub fn sub_hours(&self, hours: u64) -> Self {
        self.sub_seconds(hours * 60 * 60)
    }

    pub fn sub_one_second(t: &TimePoint) -> TimePoint {
        if t.second > 0 {
            return TimePoint {
                second: t.second - 1,
                millisecond: 0,
                precision: t.precision.clone(),
                zone: t.zone.clone(),
                ..t.clone()
            };
        }

        if t.minute > 0 {
            return TimePoint {
                minute: t.minute - 1,
                second: 59,
                precision: t.precision.clone(),
                zone: t.zone.clone(),
                ..t.clone()
            };
        }

        if t.hour > 0 {
            return TimePoint {
                hour: t.hour - 1,
                minute: 59,
                second: 59,
                precision: t.precision.clone(),
                zone: t.zone.clone(),
                ..t.clone()
            };
        }

        let previous_day = Self::sub_one_day(t);

        let second = if is_leap_second(previous_day.year, previous_day.month, previous_day.day) {
            60
        } else {
            59
        };

        TimePoint {
            hour: 23,
            minute: 59,
            second,
            precision: t.precision.clone(),
            zone: t.zone.clone(),
            ..previous_day
        }
    }

    pub fn sub_one_day(t: &TimePoint) -> TimePoint {
        if t.day > 1 {
            return TimePoint {
                day: t.day - 1,
                hour: 0,
                minute: 0,
                second: 0,
                precision: t.precision.clone(),
                ..t.clone()
            };
        }

        if t.month > 1 {
            let previous_month = t.month - 1;
            let last_day = days_in_month(t.year, previous_month);

            return TimePoint {
                month: previous_month,
                day: last_day,
                hour: 0,
                minute: 0,
                second: 0,
                precision: t.precision.clone(),
                ..t.clone()
            };
        }

        TimePoint {
            year: t.year - 1,
            month: 12,
            day: 31,
            hour: 0,
            minute: 0,
            second: 0,
            precision: t.precision.clone(),
            ..t.clone()
        }
    }

    pub fn duration_until(&self, other: &TimePoint) -> Result<i64, String> {
        Ok(other.to_unix_timestamp()? - self.to_unix_timestamp()?)
    }

    pub fn add_duration(&self, duration: Duration) -> Self {
        if duration.is_negative() {
            return self.sub_duration(-duration);
        }

        let seconds = duration.seconds() as u64;
        let subsec = duration.subsec_milliseconds();
        let total_milliseconds = self.millisecond as i64 + subsec;
        let carry = total_milliseconds / 1000;
        let remainder = total_milliseconds % 1000;

        let base = self.add_seconds(seconds + carry as u64);

        TimePoint {
            millisecond: remainder as u32,
            ..base
        }
    }

    pub fn sub_duration(&self, duration: Duration) -> Self {
        if duration.is_negative() {
            return self.add_duration(-duration);
        }

        let seconds = duration.seconds() as u64;
        let subsec = duration.subsec_milliseconds();
        let mut delta = self.millisecond as i64 - subsec;
        let mut borrow = 0;

        if delta < 0 {
            delta += 1000;
            borrow = 1;
        }

        let base = self.sub_seconds(seconds + borrow);

        TimePoint {
            millisecond: delta as u32,
            ..base
        }
    }

    pub fn duration_since(&self, earlier: &TimePoint) -> Duration {
        if self == earlier {
            return Duration::zero();
        }

        if self > earlier {
            let seconds = seconds_forward(earlier, &self.with_millisecond(0));
            Duration::from_milliseconds(
                seconds as i64 * 1000 + self.millisecond as i64 - earlier.millisecond as i64,
            )
        } else {
            -earlier.duration_since(self)
        }
    }

    fn with_millisecond(&self, millisecond: u32) -> Self {
        TimePoint {
            millisecond,
            ..self.clone()
        }
    }

    pub fn add_period(&self, period: Period) -> Self {
        let uncertainty = self.uncertainty;
        let mut result = self.clone();

        result = add_years_calendar(&result, period.years());
        result = add_months_calendar(&result, period.months());
        result = add_days_signed(&result, period.days());

        let total_milliseconds = period.hours() * 3_600_000
            + period.minutes() * 60_000
            + period.seconds() * 1000
            + period.milliseconds();

        result = result.add_duration(Duration::from_milliseconds(total_milliseconds));

        result.uncertainty = uncertainty;
        result
    }

    pub fn sub_period(&self, period: Period) -> Self {
        self.add_period(-period)
    }
}

fn add_years_calendar(t: &TimePoint, years: i64) -> TimePoint {
    if years == 0 {
        return t.clone();
    }

    let year = t.year as i64 + years;
    let day = t.day.min(days_in_month(year as u32, t.month));

    TimePoint {
        year: year as u32,
        day,
        ..t.clone()
    }
}

fn add_months_calendar(t: &TimePoint, months: i64) -> TimePoint {
    if months == 0 {
        return t.clone();
    }

    let total = t.month as i64 - 1 + months;
    let year = t.year as i64 + total.div_euclid(12);
    let month = total.rem_euclid(12) + 1;
    let day = t.day.min(days_in_month(year as u32, month as u32));

    TimePoint {
        year: year as u32,
        month: month as u32,
        day,
        ..t.clone()
    }
}

fn add_days_signed(t: &TimePoint, days: i64) -> TimePoint {
    let mut result = t.clone();

    for _ in 0..days.unsigned_abs() {
        if days < 0 {
            result = TimePoint::sub_one_day(&result);
        } else {
            result = TimePoint::add_one_day(&result);
        }
    }

    TimePoint {
        hour: t.hour,
        minute: t.minute,
        second: t.second,
        millisecond: t.millisecond,
        ..result
    }
}

fn seconds_forward(from: &TimePoint, to: &TimePoint) -> u64 {
    let mut count = 0u64;
    let mut current = from.clone();

    while !same_second_position(&current, to) {
        current = TimePoint::add_one_second(&current);
        count += 1;
    }

    count
}

fn same_second_position(a: &TimePoint, b: &TimePoint) -> bool {
    a.year == b.year
        && a.month == b.month
        && a.day == b.day
        && a.hour == b.hour
        && a.minute == b.minute
        && a.second == b.second
}

impl Add<Duration> for TimePoint {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self {
        self.add_duration(rhs)
    }
}

impl Sub<Duration> for TimePoint {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self {
        self.sub_duration(rhs)
    }
}

impl Sub for TimePoint {
    type Output = Duration;

    fn sub(self, rhs: TimePoint) -> Duration {
        self.duration_since(&rhs)
    }
}

impl Add<Period> for TimePoint {
    type Output = Self;

    fn add(self, rhs: Period) -> Self {
        self.add_period(rhs)
    }
}

impl Sub<Period> for TimePoint {
    type Output = Self;

    fn sub(self, rhs: Period) -> Self {
        self.sub_period(rhs)
    }
}

impl TimePoint {
    pub fn equals(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self, None)?;
        let b = to_interval(other, None)?;

        Ok(a.equals(&b))
    }

    pub fn before(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self, None)?;
        let b = to_interval(other, None)?;

        Ok(a.before(&b))
    }

    pub fn after(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self, None)?;
        let b = to_interval(other, None)?;

        Ok(a.after(&b))
    }

    pub fn contains(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self, None)?;
        let b = to_interval(other, None)?;

        Ok(a.contains(&b))
    }

    pub fn overlaps(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self, None)?;
        let b = to_interval(other, None)?;

        Ok(a.overlaps(&b))
    }

    pub fn allen_relation(&self, other: &TimePoint) -> Result<AllenRelation, String> {
        let a = to_interval(self, None)?;
        let b = to_interval(other, None)?;

        a.allen_relation(&b)
    }
}

pub fn encode_date(year: u32, month: u32, day: u32) -> u32 {
    (year << 9) | (month << 5) | day
}

pub fn decode_year(encoded: u32) -> u32 {
    encoded >> 9
}

pub fn decode_month(encoded: u32) -> u32 {
    (encoded >> 5) & 0b1111
}

pub fn decode_day(encoded: u32) -> u32 {
    encoded & 0b11111
}

pub fn encode_datetime(
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
) -> u64 {
    ((year as u64) << 50)
        | ((month as u64) << 46)
        | ((day as u64) << 41)
        | ((hour as u64) << 36)
        | ((minute as u64) << 30)
        | ((second as u64) << 24)
        | millisecond as u64
}
