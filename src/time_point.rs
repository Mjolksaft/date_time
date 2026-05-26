use std::cmp::Ordering;
use time::OffsetDateTime;

use crate::leap_second::is_leap_second;
use crate::precision::Precision;
use crate::time_zone::TimeZone;
use crate::util::{days_in_month, valid_date};
use crate::truth_values::TruthValue;
use crate::interval::{to_interval};

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

    // future optimization:
    // boundary_key: u64,
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
    Self::new_with_zone(
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
            TimeZone::default(),
        )
    }

    pub fn new_with_zone(
        year: u32,
        month: Option<u32>,
        day: Option<u32>,
        hour: Option<u32>,
        minute: Option<u32>,
        second: Option<u32>,
        millisecond: Option<u32>,
        zone: TimeZone,
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

            if !zone.supports_leap_seconds() {
                return Err(String::from("This time zone does not support leap seconds"));
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
            zone,
        })
    }
    
    pub fn now_utc() -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            year: now.year() as u32,
            month: now.month() as u32,
            day: now.day() as u32,
            hour: now.hour() as u32,
            minute: now.minute() as u32,
            second: now.second() as u32,
            millisecond: now.nanosecond() / 1_000_000,
            precision: Precision::Millisecond,
            zone: TimeZone::UTC,
        }
    }

    pub fn add_one_millisecond(t: &TimePoint) -> TimePoint {
        if t.millisecond == 999 {
            let next_second = Self::add_one_second(t);

            return TimePoint {
                millisecond: 0,
                precision: t.precision.clone(),
                zone: t.zone,
                ..next_second
            };
        }

        TimePoint {
            millisecond: t.millisecond + 1,
            ..t.clone()
        }
    }

    pub fn add_one_second(t: &TimePoint) -> TimePoint {
        if t.second == 59 && is_leap_second(t.year, t.month, t.day) {
            return TimePoint {
                second: 60,
                millisecond: 0,
                ..t.clone()
            };
        }

        if t.second == 59 || t.second == 60 {
            let next_minute = Self::add_one_minute(t);

            return TimePoint {
                millisecond: 0,
                precision: t.precision.clone(),
                zone: t.zone,
                ..next_minute
            };
        }

        TimePoint {
            second: t.second + 1,
            millisecond: 0,
            ..t.clone()
        }
    }

    pub fn add_milliseconds(&self, milliseconds: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..milliseconds {
            result = Self::add_one_millisecond(&result);
        }

        result
    }

    pub fn add_seconds(&self, seconds: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..seconds {
            result = Self::add_one_second(&result);
        }

        result
    }

    pub fn add_minutes(&self, minutes: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..minutes {
            result = Self::add_one_minute(&result);
        }

        result
    }

    pub fn add_hours(&self, hours: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..hours {
            result = Self::add_one_hour(&result);
        }

        result
    }

    pub fn add_days(&self, days: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..days {
            result = Self::add_one_day(&result);
        }

        result
    }

    pub fn add_months(&self, months: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..months {
            result = Self::add_one_month(&result);
        }

        result
    }

    pub fn add_years(&self, years: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..years {
            result = Self::add_one_year(&result);
        }

        result
    }

    pub fn add_one_minute(t: &TimePoint) -> TimePoint {
        if t.minute == 59 {
            let next_hour = Self::add_one_hour(t);

            return TimePoint {
                precision: t.precision.clone(),
                zone: t.zone,
                ..next_hour
            };
        }

        TimePoint {
            minute: t.minute + 1,
            second: 0,
            millisecond: 0,
            ..t.clone()
        }
    }

    pub fn add_one_hour(t: &TimePoint) -> TimePoint {
        if t.hour == 23 {
            let next_day = Self::add_one_day(t);

            return TimePoint {
                precision: t.precision.clone(),
                zone: t.zone,
                ..next_day
            };
        }

        TimePoint {
            hour: t.hour + 1,
            minute: 0,
            second: 0,
            millisecond: 0,
            ..t.clone()
        }
    }

    pub fn add_one_day(t: &TimePoint) -> TimePoint {
        let days = days_in_month(t.year, t.month)
        .expect("TimePoint should always contain a valid month");

        if t.day == days {
            if t.month == 12 {
                return start_of(t.year + 1, 1, 1, t.precision.clone());
            }

            return start_of(t.year, t.month + 1, 1, t.precision.clone());
        }

        start_of(t.year, t.month, t.day + 1, t.precision.clone())
    }

    pub fn add_one_month(t: &TimePoint) -> TimePoint {
        if t.month == 12 {
            return start_of(t.year + 1, 1, 1, t.precision.clone());
        }

        start_of(t.year, t.month + 1, 1, t.precision.clone())
    }

    pub fn add_one_year(t: &TimePoint) -> TimePoint {
        start_of(t.year + 1, 1, 1, t.precision.clone())
    }

    pub fn sub_milliseconds(&self, milliseconds: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..milliseconds {
            result = Self::sub_one_millisecond(&result);
        }

        result
    }

    pub fn sub_seconds(&self, seconds: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..seconds {
            result = Self::sub_one_second(&result);
        }

        result
    }

    pub fn sub_minutes(&self, minutes: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..minutes {
            result = Self::sub_one_minute(&result);
        }

        result
    }

    pub fn sub_hours(&self, hours: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..hours {
            result = Self::sub_one_hour(&result);
        }

        result
    }

    pub fn sub_days(&self, days: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..days {
            result = Self::sub_one_day(&result);
        }

        result
    }

    pub fn sub_months(&self, months: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..months {
            result = Self::sub_one_month(&result);
        }

        result
    }

    pub fn sub_years(&self, years: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..years {
            result = Self::sub_one_year(&result);
        }

        result
    }

    pub fn sub_one_millisecond(t: &TimePoint) -> TimePoint {
        if t.millisecond > 0 {
            return TimePoint {
                millisecond: t.millisecond - 1,
                ..t.clone()
            };
        }

        let previous_second = Self::sub_one_second(t);

        TimePoint {
            millisecond: 999,
            precision: t.precision.clone(),
            zone: t.zone,
            ..previous_second
        }
    }

    pub fn sub_one_second(t: &TimePoint) -> TimePoint {
        if t.second > 0 {
            return TimePoint {
                second: t.second - 1,
                millisecond: 0,
                ..t.clone()
            };
        }

        let previous_minute = Self::sub_one_minute(t);

        let second = if is_leap_second(
            previous_minute.year,
            previous_minute.month,
            previous_minute.day,
        ) {
            60
        } else {
            59
        };

        TimePoint {
            second,
            millisecond: 0,
            precision: t.precision.clone(),
            zone: t.zone,
            ..previous_minute
        }
    }

    pub fn sub_one_minute(t: &TimePoint) -> TimePoint {
        if t.minute > 0 {
            return TimePoint {
                minute: t.minute - 1,
                second: 59,
                millisecond: 0,
                ..t.clone()
            };
        }

        let previous_hour = Self::sub_one_hour(t);

        TimePoint {
            minute: 59,
            second: 59,
            millisecond: 0,
            precision: t.precision.clone(),
            zone: t.zone,
            ..previous_hour
        }
    }

    pub fn sub_one_hour(t: &TimePoint) -> TimePoint {
        if t.hour > 0 {
            return TimePoint {
                hour: t.hour - 1,
                minute: 59,
                second: 59,
                millisecond: 0,
                ..t.clone()
            };
        }

        let previous_day = Self::sub_one_day(t);

        TimePoint {
            hour: 23,
            minute: 59,
            second: 59,
            millisecond: 0,
            precision: t.precision.clone(),
            zone: t.zone,
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
                millisecond: 0,
                ..t.clone()
            };
        }

        if t.month > 1 {
            let previous_month = t.month - 1;

            let last_day = days_in_month(t.year, previous_month)
                .expect("TimePoint should always contain a valid month");

            return TimePoint {
                month: previous_month,
                day: last_day,
                hour: 0,
                minute: 0,
                second: 0,
                millisecond: 0,
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
            millisecond: 0,
            ..t.clone()
        }
    }

    pub fn sub_one_month(t: &TimePoint) -> TimePoint {
        if t.month > 1 {
            return start_of(
                t.year,
                t.month - 1,
                1,
                t.precision.clone(),
            );
        }

        start_of(
            t.year - 1,
            12,
            1,
            t.precision.clone(),
        )
    }

    pub fn sub_one_year(t: &TimePoint) -> TimePoint {
        start_of(
            t.year - 1,
            1,
            1,
            t.precision.clone(),
        )
    }
    pub fn as_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}-{:02}-{:02}-{:02}-{:03}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.millisecond
        )
    }

    pub fn equals(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self)?;
        let b = to_interval(other)?;

        Ok(a.equals(&b))
    }

    pub fn before(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self)?;
        let b = to_interval(other)?;

        Ok(a.before(&b))
    }

    pub fn after(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self)?;
        let b = to_interval(other)?;

        Ok(a.after(&b))
    }

    pub fn contains(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self)?;
        let b = to_interval(other)?;

        Ok(a.contains(&b))
    }

    pub fn overlaps(&self, other: &TimePoint) -> Result<TruthValue, String> {
        let a = to_interval(self)?;
        let b = to_interval(other)?;

        Ok(a.overlaps(&b))
    }
}

pub fn time_point(input: &str) -> Result<TimePoint, String> {
    if input.is_empty() {
        return Err(String::from("No args"));
    }

    let parsed = parse_date_time_point(input)?;

    match parsed.len() {
        1 => TimePoint::new(parsed[0], None, None, None, None, None, None),
        2 => TimePoint::new(parsed[0], Some(parsed[1]), None, None, None, None, None),
        3 => TimePoint::new(parsed[0], Some(parsed[1]), Some(parsed[2]), None, None, None, None),
        4 => TimePoint::new(parsed[0], Some(parsed[1]), Some(parsed[2]), Some(parsed[3]), None, None, None),
        5 => TimePoint::new(parsed[0], Some(parsed[1]), Some(parsed[2]), Some(parsed[3]), Some(parsed[4]), None, None),
        6 => TimePoint::new(parsed[0], Some(parsed[1]), Some(parsed[2]), Some(parsed[3]), Some(parsed[4]), Some(parsed[5]), None),
        7 => TimePoint::new(parsed[0], Some(parsed[1]), Some(parsed[2]), Some(parsed[3]), Some(parsed[4]), Some(parsed[5]), Some(parsed[6])),
        _ => Err(String::from("Invalid date format")),
    }
}

pub fn parse_date_time_point(input: &str) -> Result<Vec<u32>, String> {
    input
        .split('-')
        .map(|x| x.parse::<u32>().map_err(|_| String::from("Invalid number format")))
        .collect()
}

fn start_of(year: u32, month: u32, day: u32, precision: Precision) -> TimePoint {
    TimePoint {
        year,
        month,
        day,
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
        precision,
        zone: TimeZone::UTC,
    }
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