use std::fmt;
use std::ops::{Add, Neg, Sub};

/// A fixed elapsed amount of time with millisecond resolution and no calendar
/// concepts (months and years are not representable).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Duration {
    milliseconds: i64,
}

impl Duration {
    pub fn from_milliseconds(milliseconds: i64) -> Self {
        Self { milliseconds }
    }

    pub fn from_seconds(seconds: i64) -> Self {
        Self {
            milliseconds: seconds
                .checked_mul(1000)
                .expect("Duration overflow in from_seconds"),
        }
    }

    pub fn from_minutes(minutes: i64) -> Self {
        Self::from_seconds(
            minutes
                .checked_mul(60)
                .expect("Duration overflow in from_minutes"),
        )
    }

    pub fn from_hours(hours: i64) -> Self {
        Self::from_seconds(
            hours
                .checked_mul(3600)
                .expect("Duration overflow in from_hours"),
        )
    }

    pub fn zero() -> Self {
        Self { milliseconds: 0 }
    }

    pub fn milliseconds(self) -> i64 {
        self.milliseconds
    }

    pub fn seconds(self) -> i64 {
        self.milliseconds / 1000
    }

    pub fn subsec_milliseconds(self) -> i64 {
        self.milliseconds % 1000
    }

    pub fn is_zero(self) -> bool {
        self.milliseconds == 0
    }

    pub fn is_negative(self) -> bool {
        self.milliseconds < 0
    }

    pub fn is_positive(self) -> bool {
        self.milliseconds > 0
    }

    pub fn checked_add(self, other: Duration) -> Option<Duration> {
        Some(Duration::from_milliseconds(
            self.milliseconds.checked_add(other.milliseconds)?,
        ))
    }

    pub fn checked_sub(self, other: Duration) -> Option<Duration> {
        Some(Duration::from_milliseconds(
            self.milliseconds.checked_sub(other.milliseconds)?,
        ))
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        self.checked_add(rhs).expect("Duration addition overflow")
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        self.checked_sub(rhs)
            .expect("Duration subtraction overflow")
    }
}

impl Neg for Duration {
    type Output = Duration;

    fn neg(self) -> Duration {
        Duration::from_milliseconds(
            self.milliseconds
                .checked_neg()
                .expect("Duration negation overflow"),
        )
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.milliseconds == 0 {
            return write!(f, "0s");
        }

        let sign = if self.milliseconds < 0 { "-" } else { "" };
        let magnitude = self.milliseconds.unsigned_abs();
        let whole = magnitude / 1000;
        let fraction = magnitude % 1000;

        if fraction == 0 {
            write!(f, "{sign}{whole}s")
        } else {
            write!(f, "{sign}{whole}.{fraction:03}s")
        }
    }
}
