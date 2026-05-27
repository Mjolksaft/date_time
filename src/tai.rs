use crate::leap_second::get_leap_seconds_data;
use crate::time_point::TimePoint;
use crate::time_zone::TimeZone;

const TAI_UTC_OFFSET_AT_1972: u64 = 10;

fn leap_seconds_before_date(year: u32, month: u32, day: u32) -> u64 {
    let leap_days = get_leap_seconds_data();

    leap_days
        .iter()
        .filter(|&&(ly, lm, ld)| {
            (ly, lm, ld) >= (1972, 1, 1)
                && (ly, lm, ld) < (year, month, day)
        })
        .count() as u64
}

pub fn tai_utc_offset_seconds(utc: &TimePoint) -> Result<u64, String> {
    if utc.zone != TimeZone::UTC {
        return Err(String::from("TAI offset can only be calculated from UTC"));
    }

    Ok(TAI_UTC_OFFSET_AT_1972
        + leap_seconds_before_date(utc.year, utc.month, utc.day))
}

pub fn utc_to_tai(utc: &TimePoint) -> Result<TimePoint, String> {
    if utc.zone != TimeZone::UTC {
        return Err(String::from("Input must be UTC"));
    }

    let offset = tai_utc_offset_seconds(utc)?;

    let mut result = utc.clone();

    for _ in 0..offset {
        result = TimePoint::add_one_second(&result);
    }

    result.zone = TimeZone::TAI;
    result.precision = utc.precision.clone();

    if result.second == 60 {
        result = TimePoint {
            second: 0,
            zone: TimeZone::TAI,
            precision: utc.precision.clone(),
            ..TimePoint::add_one_second(&result)
        };
    }

    Ok(result)
}

pub fn tai_to_utc(tai: &TimePoint) -> Result<TimePoint, String> {
    if tai.zone != TimeZone::TAI {
        return Err(String::from("Input must be TAI"));
    }

    if tai.second == 60 {
        return Err(String::from("TAI does not support leap seconds"));
    }

    for offset in 10..=60 {
        let mut candidate = tai.clone();
        candidate.zone = TimeZone::UTC;

        candidate = candidate.sub_seconds(offset);
        candidate.zone = TimeZone::UTC;
        candidate.precision = tai.precision.clone();

        let converted_back = utc_to_tai(&candidate)?;

        if same_instant_fields(&converted_back, tai) {
            return Ok(candidate);
        }
    }

    Err(String::from("Could not convert TAI to UTC"))
}

fn same_instant_fields(a: &TimePoint, b: &TimePoint) -> bool {
    a.year == b.year
        && a.month == b.month
        && a.day == b.day
        && a.hour == b.hour
        && a.minute == b.minute
        && a.second == b.second
        && a.millisecond == b.millisecond
}