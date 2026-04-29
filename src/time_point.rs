use time::OffsetDateTime;
use std::cmp::Ordering;

use crate::precision::Precision;
use crate::util::valid_date;
use crate::interval::to_interval;
use crate::truth_values::TruthValue;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimePoint {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub precision: Precision,
}

impl TimePoint {
    pub fn new(
        year: u32,
        month: Option<u32>,
        day: Option<u32>,
        hour: Option<u32>,
        minute: Option<u32>,
        second: Option<u32>,
    ) -> Result<Self, String> {
        let precision = match (month, day, hour, minute, second) {
            (None, None, None, None, None) => Precision::Year,
            (Some(_), None, None, None, None) => Precision::Month,
            (Some(_), Some(_), None, None, None) => Precision::Day,
            (Some(_), Some(_), Some(_), None, None) => Precision::Hour,
            (Some(_), Some(_), Some(_), Some(_), None) => Precision::Minute,
            (Some(_), Some(_), Some(_), Some(_), Some(_)) => Precision::Second,
            _ => return Err(String::from("Invalid missing fields order")),
        };

        let month = month.unwrap_or(1);
        let day = day.unwrap_or(1);
        let hour = hour.unwrap_or(0);
        let minute = minute.unwrap_or(0);
        let second = second.unwrap_or(0);

        valid_date(year, Some(month), Some(day), Some(hour), Some(minute), Some(second))?;

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            precision,
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
            precision: Precision::Second,
        }
    }
}

pub fn time_point(input: &str) -> Result<TimePoint, String> {
    if input.is_empty() {
        return Err(String::from("No args"));
    }

    let parsed_date: Vec<u32> = parse_date_time_point(input)?;

    match parsed_date.len() {
        1 => TimePoint::new(parsed_date[0], None, None, None, None, None),

        2 => TimePoint::new(parsed_date[0], Some(parsed_date[1]), None, None, None, None),

        3 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            Some(parsed_date[2]),
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
        ),

        5 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            Some(parsed_date[2]),
            Some(parsed_date[3]),
            Some(parsed_date[4]),
            None,
        ),

        6 => TimePoint::new(
            parsed_date[0],
            Some(parsed_date[1]),
            Some(parsed_date[2]),
            Some(parsed_date[3]),
            Some(parsed_date[4]),
            Some(parsed_date[5]),
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
        )
            .cmp(&(
                other.year,
                other.month,
                other.day,
                other.hour,
                other.minute,
                other.second,
            ))
    }
}

impl PartialOrd for TimePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TimePoint {
    pub fn boundary_key(&self) -> u64 {
        encode_datetime(
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )
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
) -> u64 {
    ((year as u64) << 26)
        | ((month as u64) << 22)
        | ((day as u64) << 17)
        | ((hour as u64) << 12)
        | ((minute as u64) << 6)
        | second as u64
}