use crate::precision::Precision;
use std::cmp::Ordering;

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

pub fn parse_date_time_point(input: &str) -> Result<Vec<u32>, String> {
    let parts: Vec<&str> = input.split("-").collect();
    
    parts
        .iter()
        .map(|x| x.parse::<u32>().map_err(|_| String::from("Invalid number format")))
        .collect()
}

pub fn time_point(input: &str) -> Result<TimePoint, String> {
    if input.is_empty() {
        return Err(String::from("No args"));
    }

    let parsed_date: Vec<u32> = parse_date_time_point(input)?;

    match parsed_date.len() {
        1 => Ok(TimePoint {
            year: parsed_date[0],
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            precision: Precision::Year,
        }),

        2 => {
            valid_date(parsed_date[0], Some(parsed_date[1]), None)?;

            Ok(TimePoint {
                year: parsed_date[0],
                month: parsed_date[1],
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                precision: Precision::Month,
            })
        },

        3 => {
            valid_date(parsed_date[0], Some(parsed_date[1]), Some(parsed_date[2]))?;

            Ok(TimePoint {
                year: parsed_date[0],
                month: parsed_date[1],
                day: parsed_date[2],
                hour: 0,
                minute: 0,
                second: 0,
                precision: Precision::Day,
            })
        },

        4 => {
            valid_date(parsed_date[0], Some(parsed_date[1]), Some(parsed_date[2]))?;

            if parsed_date[3] > 23 {
                return Err(String::from("Invalid hour"));
            }

            Ok(TimePoint {
                year: parsed_date[0],
                month: parsed_date[1],
                day: parsed_date[2],
                hour: parsed_date[3],
                minute: 0,
                second: 0,
                precision: Precision::Hour,
            })
        }

        5 => {
            valid_date(parsed_date[0], Some(parsed_date[1]), Some(parsed_date[2]))?;

            if parsed_date[3] > 23 {
                return Err(String::from("Invalid hour"));
            }

            if parsed_date[4] > 59 {
                return Err(String::from("Invalid minute"));
            }

            Ok(TimePoint {
                year: parsed_date[0],
                month: parsed_date[1],
                day: parsed_date[2],
                hour: parsed_date[3],
                minute: parsed_date[4],
                second: 0,
                precision: Precision::Minute,
            })
        }

        6 => {
            valid_date(parsed_date[0], Some(parsed_date[1]), Some(parsed_date[2]))?;

            if parsed_date[3] > 23 {
                return Err(String::from("Invalid hour"));
            }

            if parsed_date[4] > 59 {
                return Err(String::from("Invalid minute"));
            }

            if parsed_date[5] > 59 {
                return Err(String::from("Invalid second"));
            }

            Ok(TimePoint {
                year: parsed_date[0],
                month: parsed_date[1],
                day: parsed_date[2],
                hour: parsed_date[3],
                minute: parsed_date[4],
                second: parsed_date[5],
                precision: Precision::Second,
            })
        }

        _ => Err(String::from("Invalid date format")),
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