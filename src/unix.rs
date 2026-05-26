use crate::precision::Precision;
use crate::time_point::TimePoint;
use crate::time_zone::TimeZone;

pub fn to_unix_timestamp(t: &TimePoint) -> Result<i64, String> {
    if t.second == 60 {
        return Err(String::from("Unix timestamp cannot represent leap seconds"));
    }

    let date = time::Date::from_calendar_date(
        t.year as i32,
        time::Month::try_from(t.month as u8)
            .map_err(|_| String::from("Invalid month"))?,
        t.day as u8,
    )
    .map_err(|_| String::from("Invalid date"))?;

    let time = time::Time::from_hms_milli(
        t.hour as u8,
        t.minute as u8,
        t.second as u8,
        t.millisecond as u16,
    )
    .map_err(|_| String::from("Invalid time"))?;

    let datetime = time::PrimitiveDateTime::new(date, time);

    Ok(datetime.assume_utc().unix_timestamp())
}

pub fn from_unix_timestamp(ts: i64) -> Result<TimePoint, String> {
    let dt = time::OffsetDateTime::from_unix_timestamp(ts)
        .map_err(|_| String::from("Invalid Unix timestamp"))?;

    Ok(TimePoint {
        year: dt.year() as u32,
        month: dt.month() as u32,
        day: dt.day() as u32,
        hour: dt.hour() as u32,
        minute: dt.minute() as u32,
        second: dt.second() as u32,
        millisecond: dt.nanosecond() / 1_000_000,
        precision: Precision::Second,
        zone: TimeZone::Unix,
    })
}