use std::fmt;
use std::ops::{Add, Neg, Sub};

/// A calendar-relative amount of time (years, months, days, and a fixed time
/// part). Unlike [`crate::duration::Duration`], it only becomes concrete once
/// anchored to a [`TimePoint`](crate::time_point::TimePoint).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Period {
    years: i64,
    months: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
}

// Euclidean division that keeps negatives canonical (e.g. -1s -> 0 min -1s).
fn divmod(value: i64, divisor: i64) -> (i64, i64) {
    (value.div_euclid(divisor), value.rem_euclid(divisor))
}

/// Re-normalizes components so each field is in range: 12 months -> 1 year,
/// 1000 ms -> 1 s, and so on, carrying into the next coarser field.
fn normalized(
    years: i64,
    months: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
) -> Period {
    let (sec_carry, ms) = divmod(milliseconds, 1000);
    let (min_carry, sec) = divmod(seconds + sec_carry, 60);
    let (hour_carry, min) = divmod(minutes + min_carry, 60);
    let (year_carry, month) = divmod(months, 12);

    Period {
        years: years + year_carry,
        months: month,
        days,
        hours: hours + hour_carry,
        minutes: min,
        seconds: sec,
        milliseconds: ms,
    }
}

impl Period {
    pub fn new(
        years: i64,
        months: i64,
        days: i64,
        hours: i64,
        minutes: i64,
        seconds: i64,
        milliseconds: i64,
    ) -> Self {
        normalized(years, months, days, hours, minutes, seconds, milliseconds)
    }

    pub fn from_years(years: i64) -> Self {
        Self::new(years, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_months(months: i64) -> Self {
        Self::new(0, months, 0, 0, 0, 0, 0)
    }

    pub fn from_days(days: i64) -> Self {
        Self::new(0, 0, days, 0, 0, 0, 0)
    }

    pub fn from_hours(hours: i64) -> Self {
        Self::new(0, 0, 0, hours, 0, 0, 0)
    }

    pub fn from_minutes(minutes: i64) -> Self {
        Self::new(0, 0, 0, 0, minutes, 0, 0)
    }

    pub fn from_seconds(seconds: i64) -> Self {
        Self::new(0, 0, 0, 0, 0, seconds, 0)
    }

    pub fn from_milliseconds(milliseconds: i64) -> Self {
        Self::new(0, 0, 0, 0, 0, 0, milliseconds)
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0)
    }

    pub fn years(self) -> i64 {
        self.years
    }

    pub fn months(self) -> i64 {
        self.months
    }

    pub fn days(self) -> i64 {
        self.days
    }

    pub fn hours(self) -> i64 {
        self.hours
    }

    pub fn minutes(self) -> i64 {
        self.minutes
    }

    pub fn seconds(self) -> i64 {
        self.seconds
    }

    pub fn milliseconds(self) -> i64 {
        self.milliseconds
    }

    pub fn is_zero(self) -> bool {
        self == Period::zero()
    }

    pub fn is_negative(self) -> bool {
        for component in [
            self.years,
            self.months,
            self.days,
            self.hours,
            self.minutes,
            self.seconds,
            self.milliseconds,
        ] {
            if component != 0 {
                return component < 0;
            }
        }

        false
    }
}

impl Add for Period {
    type Output = Period;

    fn add(self, rhs: Period) -> Period {
        Period::new(
            self.years + rhs.years,
            self.months + rhs.months,
            self.days + rhs.days,
            self.hours + rhs.hours,
            self.minutes + rhs.minutes,
            self.seconds + rhs.seconds,
            self.milliseconds + rhs.milliseconds,
        )
    }
}

impl Sub for Period {
    type Output = Period;

    fn sub(self, rhs: Period) -> Period {
        self + (-rhs)
    }
}

impl Neg for Period {
    type Output = Period;

    fn neg(self) -> Period {
        Period::new(
            -self.years,
            -self.months,
            -self.days,
            -self.hours,
            -self.minutes,
            -self.seconds,
            -self.milliseconds,
        )
    }
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negative = self.is_negative();
        let years = self.years.unsigned_abs();
        let months = self.months.unsigned_abs();
        let days = self.days.unsigned_abs();
        let hours = self.hours.unsigned_abs();
        let minutes = self.minutes.unsigned_abs();
        let seconds = self.seconds.unsigned_abs();
        let milliseconds = self.milliseconds.unsigned_abs();

        let mut out = String::new();
        if negative {
            out.push('-');
        }
        out.push('P');

        if years > 0 {
            out.push_str(&format!("{years}Y"));
        }
        if months > 0 {
            out.push_str(&format!("{months}M"));
        }
        if days > 0 {
            out.push_str(&format!("{days}D"));
        }

        if hours > 0 || minutes > 0 || seconds > 0 || milliseconds > 0 {
            out.push('T');
            if hours > 0 {
                out.push_str(&format!("{hours}H"));
            }
            if minutes > 0 {
                out.push_str(&format!("{minutes}M"));
            }
            if seconds > 0 || milliseconds > 0 {
                if milliseconds > 0 {
                    out.push_str(&format!("{seconds}.{milliseconds:03}S"));
                } else {
                    out.push_str(&format!("{seconds}S"));
                }
            }
        }

        if out.ends_with('P') {
            out.push_str("0D");
        }

        write!(f, "{out}")
    }
}
