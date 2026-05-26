#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TimeZone {
    UTC,
    TAI,
    Unix,
}

impl TimeZone {
    pub fn name(&self) -> &'static str {
        match self {
            TimeZone::UTC => "UTC",
            TimeZone::TAI => "TAI",
            TimeZone::Unix => "Unix",
        }
    }

    pub fn supports_leap_seconds(&self) -> bool {
        match self {
            TimeZone::UTC => true,
            TimeZone::TAI => false,
            TimeZone::Unix => false,
        }
    }
}

impl Default for TimeZone {
    fn default() -> Self {
        TimeZone::UTC
    }
}