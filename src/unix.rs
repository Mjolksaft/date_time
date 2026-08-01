// Unix timestamp conversion.
//
// The Unix time scale counts whole seconds since the epoch (1970-01-01T00:00:00Z)
// and cannot represent leap seconds. Therefore a TimePoint carrying second 60
// cannot be converted to a Unix timestamp.
use crate::precision::Precision;
use crate::time_point::TimePoint;
use crate::time_zone::TimeZone;

pub fn to_unix_timestamp(point: &TimePoint) -> Result<i64, String> {
    if point.second == 60 {
        return Err(String::from("Unix timestamp cannot represent leap seconds"));
    }

    if point.zone == TimeZone::TAI {
        return Err(String::from(
            "TAI TimePoint must be converted to UTC before Unix conversion",
        ));
    }

    let datetime = time::PrimitiveDateTime::new(
        time::Date::from_calendar_date(
            point.year as i32,
            time::Month::try_from(point.month as u8).map_err(|_| String::from("Invalid month"))?,
            point.day as u8,
        )
        .map_err(|_| String::from("Invalid date"))?,
        time::Time::from_hms(point.hour as u8, point.minute as u8, point.second as u8)
            .map_err(|_| String::from("Invalid time"))?,
    );

    Ok(datetime.assume_utc().unix_timestamp())
}

pub fn from_unix_timestamp(ts: i64) -> TimePoint {
    let dt = time::OffsetDateTime::from_unix_timestamp(ts).unwrap();

    TimePoint {
        year: dt.year() as u32,
        month: dt.month() as u32,
        day: dt.day() as u32,
        hour: dt.hour() as u32,
        minute: dt.minute() as u32,
        second: dt.second() as u32,
        millisecond: dt.nanosecond() / 1_000_000,
        precision: Precision::Second,
        zone: TimeZone::UTC,
        uncertainty: None,
    }
}
