// TAI (Temps Atomique International) interoperability.
//
// TAI is a continuous time scale with no leap seconds, so it never displays
// second 60. UTC differs from TAI by an integer number of seconds that grows by
// one at each leap second insertion. TAI-UTC offsets come from the
// leap-seconds.list table (see leap_second.rs).
//
// During a positive leap second (23:59:60) the UTC day is one second longer
// while TAI keeps counting, so the leap second maps to a regular TAI second
// whose UTC representation is second 60.
use crate::leap_second::leap_second_entries;
use crate::precision::Precision;
use crate::time_point::TimePoint;
use crate::time_zone::TimeZone;

const UNIX_EPOCH_JULIAN_DAY: i32 = 2_440_588;
const NTP_EPOCH_SECONDS: i64 = 2_208_988_800;
const DEFAULT_OFFSET: u32 = 10;

/// Converts a TimePoint wall clock to whole seconds since the Unix epoch,
/// treating every day as exactly 86_400 seconds. Second 60 (a leap second)
/// therefore maps to the end of its day.
fn to_linear_seconds(point: &TimePoint) -> Result<i64, String> {
    let date = time::Date::from_calendar_date(
        point.year as i32,
        time::Month::try_from(point.month as u8).map_err(|_| String::from("Invalid month"))?,
        point.day as u8,
    )
    .map_err(|_| String::from("Invalid date"))?;

    let days = i64::from(date.to_julian_day()) - i64::from(UNIX_EPOCH_JULIAN_DAY);

    let mut seconds = days * 86_400;
    seconds += i64::from(point.hour) * 3_600;
    seconds += i64::from(point.minute) * 60;
    seconds += i64::from(point.second);

    Ok(seconds)
}

/// Inverse of `to_linear_seconds`: whole seconds since the Unix epoch back to a
/// wall clock. The result is a second-precision UTC TimePoint whose zone and
/// precision callers may override.
fn from_linear_seconds(seconds: i64) -> TimePoint {
    let days = seconds.div_euclid(86_400);
    let remainder = seconds.rem_euclid(86_400);

    let date = time::Date::from_julian_day(days as i32 + UNIX_EPOCH_JULIAN_DAY)
        .expect("Date out of range");

    TimePoint {
        year: date.year() as u32,
        month: date.month() as u32,
        day: date.day() as u32,
        hour: (remainder / 3_600) as u32,
        minute: ((remainder % 3_600) / 60) as u32,
        second: (remainder % 60) as u32,
        millisecond: 0,
        precision: Precision::Second,
        zone: TimeZone::UTC,
        uncertainty: None,
    }
}

/// Offset (DTAI = TAI - UTC) valid at the given NTP instant, using the
/// last entry whose timestamp is at or before it.
fn offset_at_or_before(entries: &[(u64, u32)], ntp: i64) -> u32 {
    let mut offset = DEFAULT_OFFSET;

    for (timestamp, next_offset) in entries {
        if *timestamp as i64 <= ntp {
            offset = *next_offset;
        } else {
            break;
        }
    }

    offset
}

/// Offset valid at a TAI instant. An offset `N` begins at the physical instant
/// whose UTC representation is the entry timestamp `T`, which has the TAI
/// count `T + N`. Hence offset `N` applies for TAI counts >= T + N.
fn offset_valid_at_tai(entries: &[(u64, u32)], ntp: i64) -> u32 {
    let mut offset = DEFAULT_OFFSET;

    for (timestamp, next_offset) in entries {
        if *timestamp as i64 + i64::from(*next_offset) <= ntp {
            offset = *next_offset;
        } else {
            break;
        }
    }

    offset
}

/// The TAI-UTC offset (DTAI) in seconds for the instant a TimePoint represents.
///
/// Accepts a UTC or TAI TimePoint. For a UTC leap second (second 60) the
/// offset still equals the pre-transition value: the new offset only applies
/// from the following 00:00:00.
pub fn tai_utc_offset(point: &TimePoint) -> Result<u32, String> {
    let entries = leap_second_entries();

    match point.zone {
        TimeZone::UTC => {
            let mut ntp = to_linear_seconds(point)? + NTP_EPOCH_SECONDS;

            if point.second == 60 {
                // the leap second is the last second before the transition, so
                // look it up as if it were 23:59:59
                ntp -= 1;
            }

            Ok(offset_at_or_before(&entries, ntp))
        }
        TimeZone::TAI => {
            let ntp = to_linear_seconds(point)? + NTP_EPOCH_SECONDS;
            Ok(offset_valid_at_tai(&entries, ntp))
        }
        TimeZone::Unix => Err(String::from(
            "A Unix TimePoint has no TAI-UTC relationship without UTC context",
        )),
        TimeZone::Fixed { .. } => Err(String::from(
            "A fixed-offset TimePoint must be converted to UTC before TAI conversion",
        )),
    }
}

/// Converts a UTC TimePoint to its corresponding TAI TimePoint.
///
/// Leap seconds are absorbed by the offset: the UTC leap second 23:59:60 maps
/// to a regular TAI second. The precision is preserved.
pub fn utc_to_tai(point: &TimePoint) -> Result<TimePoint, String> {
    if point.zone != TimeZone::UTC {
        return Err(String::from("utc_to_tai requires a UTC TimePoint"));
    }

    let offset = tai_utc_offset(point)?;
    let linear = to_linear_seconds(point)?;

    let mut tai = from_linear_seconds(linear + i64::from(offset));
    tai.precision = point.precision.clone();
    tai.zone = TimeZone::TAI;

    Ok(tai)
}

/// Converts a TAI TimePoint to its corresponding UTC TimePoint.
///
/// TAI never displays second 60; the TAI second that falls during a leap
/// second converts back to UTC 23:59:60. The precision is preserved.
pub fn tai_to_utc(point: &TimePoint) -> Result<TimePoint, String> {
    if point.zone != TimeZone::TAI {
        return Err(String::from("tai_to_utc requires a TAI TimePoint"));
    }

    let entries = leap_second_entries();

    let linear = to_linear_seconds(point)?;
    let ntp = linear + NTP_EPOCH_SECONDS;
    let offset = offset_valid_at_tai(&entries, ntp);

    let utc_linear = linear - i64::from(offset);

    // If the UTC wall clock lands exactly on an offset-transition boundary but
    // the TAI instant is still before the transition, this instant is the leap
    // second 23:59:60 on the day before the boundary.
    for (timestamp, next_offset) in &entries {
        if *timestamp as i64 == utc_linear + NTP_EPOCH_SECONDS
            && *timestamp as i64 + i64::from(*next_offset) > ntp
        {
            let mut leap = from_linear_seconds(utc_linear - 1);
            leap.second = 60;
            leap.precision = point.precision.clone();
            leap.zone = TimeZone::UTC;
            return Ok(leap);
        }
    }

    let mut utc = from_linear_seconds(utc_linear);
    utc.precision = point.precision.clone();
    utc.zone = TimeZone::UTC;

    Ok(utc)
}
