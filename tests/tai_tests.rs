use date_time::time_point::{time_point, TimePoint};
use date_time::time_zone::TimeZone;
use date_time::tai::{utc_to_tai, tai_to_utc, tai_utc_offset_seconds};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_to_tai_requires_utc_input() {
        let tai = TimePoint::new_with_zone(
            2027,
            Some(4),
            Some(20),
            Some(13),
            Some(45),
            Some(30),
            Some(0),
            TimeZone::TAI,
        )
        .unwrap();

        assert!(utc_to_tai(&tai).is_err());
    }

    #[test]
    fn tai_offset_for_2017_is_37_seconds() {
        let utc = time_point("2017-01-01-00-00-00").unwrap();

        let offset = tai_utc_offset_seconds(&utc).unwrap();

        assert_eq!(offset, 37);
    }

    #[test]
    fn utc_to_tai_sets_zone_to_tai() {
        let utc = time_point("2017-01-01-00-00-00").unwrap();

        let tai = utc_to_tai(&utc).unwrap();

        assert_eq!(tai.zone, TimeZone::TAI);
    }

    #[test]
    fn utc_to_tai_adds_offset_seconds() {
        let utc = time_point("2017-01-01-00-00-00").unwrap();

        let tai = utc_to_tai(&utc).unwrap();

        assert_eq!(tai.year, 2017);
        assert_eq!(tai.month, 1);
        assert_eq!(tai.day, 1);
        assert_eq!(tai.hour, 0);
        assert_eq!(tai.minute, 0);
        assert_eq!(tai.second, 37);
    }

    #[test]
    fn utc_to_tai_preserves_precision() {
        let utc = time_point("2017-01-01-00-00").unwrap();

        let tai = utc_to_tai(&utc).unwrap();

        assert_eq!(tai.precision, utc.precision);
    }

    #[test]
    fn tai_to_utc_requires_tai_input() {
        let utc = time_point("2017-01-01-00-00-37").unwrap();

        assert!(tai_to_utc(&utc).is_err());
    }

    #[test]
    fn tai_to_utc_converts_back_to_utc_zone() {
        let utc = time_point("2017-01-01-00-00-00").unwrap();
        let tai = utc_to_tai(&utc).unwrap();

        let back = tai_to_utc(&tai).unwrap();

        assert_eq!(back.zone, TimeZone::UTC);
    }

    #[test]
    fn tai_to_utc_roundtrip_preserves_utc_fields() {
        let utc = time_point("2017-01-01-00-00-00").unwrap();
        let tai = utc_to_tai(&utc).unwrap();

        let back = tai_to_utc(&tai).unwrap();

        assert_eq!(back.year, utc.year);
        assert_eq!(back.month, utc.month);
        assert_eq!(back.day, utc.day);
        assert_eq!(back.hour, utc.hour);
        assert_eq!(back.minute, utc.minute);
        assert_eq!(back.second, utc.second);
        assert_eq!(back.millisecond, utc.millisecond);
    }

    #[test]
    fn tai_to_utc_preserves_precision() {
        let utc = time_point("2017-01-01-00-00").unwrap();
        let tai = utc_to_tai(&utc).unwrap();

        let back = tai_to_utc(&tai).unwrap();

        assert_eq!(back.precision, utc.precision);
    }

    #[test]
    fn tai_to_utc_rejects_non_tai_value() {
        let result = TimePoint::new_with_zone(
            2017,
            Some(1),
            Some(1),
            Some(0),
            Some(0),
            Some(37),
            Some(0),
            TimeZone::Unix,
        )
        .unwrap();

        assert!(tai_to_utc(&result).is_err());
    }
}
