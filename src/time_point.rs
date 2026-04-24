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
            precision: Precision::Year,
        }),

        2 => {
            valid_date(parsed_date[0], Some(parsed_date[1]), None)?;

            Ok(TimePoint {
                year: parsed_date[0],
                month: parsed_date[1],
                day: 1,
                precision: Precision::Month,
            })
        }

        3 => {
            valid_date(parsed_date[0], Some(parsed_date[1]), Some(parsed_date[2]))?;

            Ok(TimePoint {
                year: parsed_date[0],
                month: parsed_date[1],
                day: parsed_date[2],
                precision: Precision::Day,
            })
        }

        _ => Err(String::from("Invalid date format")),
    }
}

impl Ord for TimePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month, self.day)
            .cmp(&(other.year, other.month, other.day))
    }
}

impl PartialOrd for TimePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
