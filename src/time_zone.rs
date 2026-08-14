//! Time zone representation.
//!
//! A `TimePoint`'s fields are its local wall clock in its `TimeZone`. `UTC`,
//! `TAI`, and `Unix` are the built-in scales, and `Fixed` represents a named
//! zone by a constant UTC offset (for example `UTC-05:00` for Eastern Time in
//! winter). Fixed-offset zones keep their offset constant; daylight-saving
//! transitions must be applied by the caller.

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimeZone {
    UTC,
    TAI,
    Unix,
    Fixed { hours: i8, minutes: i8 },
}

impl TimeZone {
    pub fn fixed(hours: i8, minutes: i8) -> Result<Self, String> {
        if !(-14..=14).contains(&hours) {
            return Err(String::from("Offset hours must be between -14 and 14"));
        }
        if !(0..=59).contains(&minutes) {
            return Err(String::from("Offset minutes must be between 0 and 59"));
        }
        if hours == 14 && minutes > 0 {
            return Err(String::from("Offset cannot exceed UTC+14:00"));
        }

        Ok(TimeZone::Fixed { hours, minutes })
    }

    /// The UTC offset in seconds for zones with a fixed relationship to UTC.
    pub fn utc_offset_seconds(&self) -> Result<i64, String> {
        match self {
            TimeZone::UTC | TimeZone::Unix => Ok(0),
            TimeZone::TAI => Err(String::from("TAI has no fixed UTC offset")),
            TimeZone::Fixed { hours, minutes } => {
                Ok(i64::from(*hours) * 3600 + i64::from(*minutes) * 60)
            }
        }
    }

    /// Offset-style label such as `+05:30` or `-05:00`.
    pub fn offset_label(&self) -> String {
        match self {
            TimeZone::UTC => String::from("+00:00"),
            TimeZone::TAI => String::from("TAI"),
            TimeZone::Unix => String::from("unix"),
            TimeZone::Fixed { hours, minutes } => {
                let sign = if *hours < 0 { "-" } else { "+" };
                format!("{sign}{:02}:{:02}", hours.unsigned_abs(), minutes)
            }
        }
    }
}

impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.offset_label())
    }
}
