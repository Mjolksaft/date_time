pub use date_time::time_point::{time_point};
pub use date_time::precision::Precision;
pub use date_time::time_zone::TimeZone;
pub use date_time::unix::{to_unix_timestamp, from_unix_timestamp};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_timepoint_to_unix_timestamp() {
        let t = time_point("2027-04-20-13-45-30").unwrap();

        let ts = to_unix_timestamp(&t);

        assert!(ts.is_ok());
    }

    #[test]
    fn unix_roundtrip_preserves_calendar_fields() {
        let t = time_point("2027-04-20-13-45-30").unwrap();

        let ts = to_unix_timestamp(&t).unwrap();
        let back = from_unix_timestamp(ts).unwrap();

        assert_eq!(back.year, 2027);
        assert_eq!(back.month, 4);
        assert_eq!(back.day, 20);
        assert_eq!(back.hour, 13);
        assert_eq!(back.minute, 45);
        assert_eq!(back.second, 30);
    }

    #[test]
    fn from_unix_timestamp_returns_unix_zone() {
        let back = from_unix_timestamp(0).unwrap();

        assert_eq!(back.zone, TimeZone::Unix);
    }

    #[test]
    fn from_unix_timestamp_returns_second_precision() {
        let back = from_unix_timestamp(0).unwrap();

        assert_eq!(back.precision, Precision::Second);
    }

    #[test]
    fn unix_timestamp_rejects_leap_second() {
        let t = time_point("2016-12-31-23-59-60").unwrap();

        let result = to_unix_timestamp(&t);

        assert!(result.is_err());
    }

    #[test]
    fn unix_epoch_is_1970_01_01() {
        let t = from_unix_timestamp(0).unwrap();

        assert_eq!(t.year, 1970);
        assert_eq!(t.month, 1);
        assert_eq!(t.day, 1);
        assert_eq!(t.hour, 0);
        assert_eq!(t.minute, 0);
        assert_eq!(t.second, 0);
    }
}