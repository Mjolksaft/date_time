use time::OffsetDateTime;
use time::convert::Millisecond;
use std::cmp::Ordering;

use crate::precision::Precision;
use crate::util::{valid_date, days_in_month};
use crate::interval::to_interval;
use crate::truth_values::TruthValue;
use crate::time_zone::TimeZone;
use crate::leap_second::is_leap_second;

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
}

fn start_of(
    year: u32,
    month: u32,
    day: u32,
    precision: Precision,
) -> TimePoint {
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
        valid_date(year, Some(month), Some(day), Some(hour), Some(minute), Some(second))?;
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
        }
    }
}

pub fn time_point(input: &str) -> Result<TimePoint, String> {
    if input.is_empty() {
        return Err(String::from("No args"));
    }

    let parsed_date: Vec<u32> = parse_date_time_point(input)?;

    match parsed_date.len() {
        1 => TimePoint::new(parsed_date[0], None, None, None, None, None, None),

        2 => TimePoint::new(parsed_date[0], Some(parsed_date[1]), None, None, None, None, None),

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
        .map(|x| x.parse::<u32>().map_err(|_| String::from("Invalid number format")))
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
        if self.second == 60 {
            return Err(String::from("Unix timestamp cannot represent leap seconds"));
        }

        let datetime = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(
                self.year as i32,
                time::Month::try_from(self.month as u8)
                    .map_err(|_| String::from("Invalid month"))?,
                self.day as u8,
            )
            .map_err(|_| String::from("Invalid date"))?,
            time::Time::from_hms(
                self.hour as u8,
                self.minute as u8,
                self.second as u8,
            )
            .map_err(|_| String::from("Invalid time"))?,
        );

        Ok(datetime.assume_utc().unix_timestamp())
    }

    pub fn from_unix_timestamp(ts: i64) -> Self {
        let dt = time::OffsetDateTime::from_unix_timestamp(ts).unwrap();

        TimePoint {
            year: dt.year() as u32,
            month: dt.month() as u32,
            day: dt.day() as u32,
            hour: dt.hour() as u32,
            minute: dt.minute() as u32,
            second: dt.second() as u32,
            millisecond: dt.nanosecond() / 1_000_000_000,
            precision: Precision::Second,
            zone: TimeZone::UTC,
        }
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

        Ok(result)
    }

    pub fn add_seconds(&self, seconds: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..seconds {
            result = Self::add_one_second(&result);
        }

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
                zone: TimeZone::UTC,
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
                zone: TimeZone::UTC,
            }
        }
    }

    pub fn add_one_day(t: &TimePoint) -> TimePoint {
        let days = days_in_month(t.year, t.month);

        if t.day == days {
            if t.month == 12 {
                start_of(t.year + 1, 1, 1, t.precision.clone())
            } else {
                start_of(t.year, t.month + 1, 1, t.precision.clone())
            }
        } else {
            start_of(t.year, t.month, t.day + 1, t.precision.clone())
        }
    }

    pub fn add_one_month(t: &TimePoint) -> TimePoint {
        if t.month == 12 {
            start_of(t.year + 1, 1, 1, t.precision.clone())
        } else {
            start_of(t.year, t.month + 1, 1, t.precision.clone())
        }
    }

    pub fn add_one_year(t: &TimePoint) -> TimePoint {
        start_of(t.year + 1, 1, 1, t.precision.clone())
    }

    pub fn sub_seconds(&self, seconds: u64) -> Self {
        let mut result = self.clone();

        for _ in 0..seconds {
            result = Self::sub_one_second(&result);
        }

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
    ((year as u64) << 26)
        | ((month as u64) << 22)
        | ((day as u64) << 17)
        | ((hour as u64) << 12)
        | ((minute as u64) << 6)
        | ((second as u64) << 10)
        | millisecond as u64
}