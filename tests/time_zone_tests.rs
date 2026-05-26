pub use date_time::time_zone::TimeZone;
pub use date_time::time_point::{TimePoint};


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn utc_has_correct_name() {
        assert_eq!(TimeZone::UTC.name(), "UTC");
    }

    #[test]
    fn tai_has_correct_name() {
        assert_eq!(TimeZone::TAI.name(), "TAI");
    }

    #[test]
    fn unix_has_correct_name() {
        assert_eq!(TimeZone::Unix.name(), "Unix");
    }

    #[test]
    fn utc_supports_leap_seconds() {
        assert_eq!(TimeZone::UTC.supports_leap_seconds(), true);
    }

    #[test]
    fn tai_does_not_support_leap_seconds() {
        assert_eq!(TimeZone::TAI.supports_leap_seconds(), false);
    }

    #[test]
    fn unix_does_not_support_leap_seconds() {
        assert_eq!(TimeZone::Unix.supports_leap_seconds(), false);
    }


    #[test]
    fn new_defaults_to_utc() {
        let t = TimePoint::new(2027, Some(4), Some(20), None, None, None, None).unwrap();

        assert_eq!(t.zone, TimeZone::UTC);
    }

    #[test]
    fn new_with_zone_sets_tai() {
        let t = TimePoint::new_with_zone(
            2027,
            Some(4),
            Some(20),
            Some(13),
            Some(45),
            Some(30),
            Some(250),
            TimeZone::TAI,
        )
        .unwrap();

        assert_eq!(t.zone, TimeZone::TAI);
    }

    #[test]
    fn new_with_zone_sets_unix() {
        let t = TimePoint::new_with_zone(
            2027,
            Some(4),
            Some(20),
            Some(13),
            Some(45),
            Some(30),
            Some(250),
            TimeZone::Unix,
        )
        .unwrap();

        assert_eq!(t.zone, TimeZone::Unix);
    }

    #[test]
    fn utc_accepts_valid_leap_second() {
        let result = TimePoint::new_with_zone(
            2016,
            Some(12),
            Some(31),
            Some(23),
            Some(59),
            Some(60),
            Some(0),
            TimeZone::UTC,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn unix_rejects_leap_second() {
        let result = TimePoint::new_with_zone(
            2016,
            Some(12),
            Some(31),
            Some(23),
            Some(59),
            Some(60),
            Some(0),
            TimeZone::Unix,
        );

        assert!(result.is_err());
    }

    #[test]
    fn tai_rejects_leap_second() {
        let result = TimePoint::new_with_zone(
            2016,
            Some(12),
            Some(31),
            Some(23),
            Some(59),
            Some(60),
            Some(0),
            TimeZone::TAI,
        );

        assert!(result.is_err());
    }
}