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