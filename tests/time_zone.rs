use date_time::time_point::time_point;
use date_time::time_zone::TimeZone;
use date_time::uncertainty::Uncertainty;

#[test]
fn fixed_validates_offsets() {
    assert_eq!(
        TimeZone::fixed(-5, 0),
        Ok(TimeZone::Fixed {
            hours: -5,
            minutes: 0
        })
    );
    assert_eq!(
        TimeZone::fixed(5, 30),
        Ok(TimeZone::Fixed {
            hours: 5,
            minutes: 30
        })
    );
    assert_eq!(
        TimeZone::fixed(14, 0),
        Ok(TimeZone::Fixed {
            hours: 14,
            minutes: 0
        })
    );

    assert!(TimeZone::fixed(15, 0).is_err());
    assert!(TimeZone::fixed(-15, 0).is_err());
    assert!(TimeZone::fixed(0, 60).is_err());
    assert!(TimeZone::fixed(14, 1).is_err());
}

#[test]
fn utc_offset_seconds() {
    assert_eq!(TimeZone::UTC.utc_offset_seconds(), Ok(0));
    assert_eq!(
        TimeZone::fixed(-5, 0).unwrap().utc_offset_seconds(),
        Ok(-18000)
    );
    assert_eq!(
        TimeZone::fixed(5, 30).unwrap().utc_offset_seconds(),
        Ok(19800)
    );
    assert!(TimeZone::TAI.utc_offset_seconds().is_err());
}

#[test]
fn offset_labels() {
    assert_eq!(TimeZone::UTC.offset_label(), "+00:00");
    assert_eq!(TimeZone::fixed(-5, 0).unwrap().offset_label(), "-05:00");
    assert_eq!(TimeZone::fixed(5, 30).unwrap().offset_label(), "+05:30");
    assert_eq!(TimeZone::fixed(-5, 0).unwrap().to_string(), "-05:00");
}

#[test]
fn with_zone_attaches_without_conversion() {
    let t = time_point("2027-04-20-12-00-00").unwrap();
    let local = t.with_zone(TimeZone::fixed(-5, 0).unwrap());

    assert_eq!(local.hour, 12);
    assert_eq!(
        local.zone,
        TimeZone::Fixed {
            hours: -5,
            minutes: 0
        }
    );
}

#[test]
fn convert_fixed_local_to_utc() {
    let local = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_zone(TimeZone::fixed(-5, 0).unwrap());

    let utc = local.to_utc().unwrap();

    assert_eq!(utc, time_point("2027-04-20-17-00-00").unwrap());
    assert_eq!(utc.zone, TimeZone::UTC);
}

#[test]
fn convert_utc_to_fixed_local() {
    let utc = time_point("2027-04-20-12-00-00").unwrap();

    let local = utc.convert_to(TimeZone::fixed(5, 30).unwrap()).unwrap();

    assert_eq!(
        local,
        time_point("2027-04-20-17-30-00")
            .unwrap()
            .with_zone(TimeZone::fixed(5, 30).unwrap())
    );
    assert_eq!(
        local.zone,
        TimeZone::Fixed {
            hours: 5,
            minutes: 30
        }
    );
}

#[test]
fn convert_across_day_boundary() {
    let local = time_point("2027-04-20-00-00-00")
        .unwrap()
        .with_zone(TimeZone::fixed(14, 0).unwrap());

    let utc = local.to_utc().unwrap();

    assert_eq!(utc, time_point("2027-04-19-10-00-00").unwrap());

    let early = time_point("2027-04-20-00-30-00")
        .unwrap()
        .with_zone(TimeZone::fixed(-5, 0).unwrap());
    assert_eq!(
        early.to_utc().unwrap(),
        time_point("2027-04-20-05-30-00").unwrap()
    );
}

#[test]
fn convert_round_trip_restores_local() {
    let local = time_point("2027-04-20-13-45-30")
        .unwrap()
        .with_zone(TimeZone::fixed(-5, 0).unwrap());

    let back = local.to_utc().unwrap().convert_to(local.zone).unwrap();

    assert_eq!(back, local);
}

#[test]
fn convert_to_same_zone_is_identity() {
    let local = time_point("2027-04-20-13-45-30")
        .unwrap()
        .with_zone(TimeZone::fixed(5, 30).unwrap());

    assert_eq!(local.convert_to(local.zone).unwrap(), local);
}

#[test]
fn convert_preserves_uncertainty() {
    let local = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_uncertainty(Uncertainty::from_seconds(10))
        .with_zone(TimeZone::fixed(-5, 0).unwrap());

    let utc = local.to_utc().unwrap();

    assert_eq!(utc.uncertainty(), Some(Uncertainty::from_seconds(10)));
    assert_eq!(utc.hour, 17);
}

#[test]
fn arithmetic_preserves_zone() {
    let t = time_point("2027-04-20-13-45-30")
        .unwrap()
        .with_zone(TimeZone::fixed(-5, 0).unwrap());

    assert_eq!(
        t.add_seconds(60).zone,
        TimeZone::Fixed {
            hours: -5,
            minutes: 0
        }
    );
    assert_eq!(
        t.add_minutes(30).zone,
        TimeZone::Fixed {
            hours: -5,
            minutes: 0
        }
    );
    assert_eq!(
        t.add_duration(date_time::duration::Duration::from_hours(1))
            .zone,
        TimeZone::Fixed {
            hours: -5,
            minutes: 0
        }
    );
    assert_eq!(
        t.add_period(date_time::period::Period::from_days(1)).zone,
        TimeZone::Fixed {
            hours: -5,
            minutes: 0
        }
    );
    assert_eq!(
        t.sub_seconds(1).zone,
        TimeZone::Fixed {
            hours: -5,
            minutes: 0
        }
    );
}

#[test]
fn arithmetic_across_local_midnight_keeps_zone() {
    let t = time_point("2027-04-20-23-59-59")
        .unwrap()
        .with_zone(TimeZone::fixed(2, 0).unwrap());

    let next = t.add_seconds(1);

    assert_eq!(next.day, 21);
    assert_eq!(
        next.zone,
        TimeZone::Fixed {
            hours: 2,
            minutes: 0
        }
    );
}

#[test]
fn unix_timestamp_accounts_for_offset() {
    let local = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_zone(TimeZone::fixed(-5, 0).unwrap());

    assert_eq!(
        local.to_unix_timestamp().unwrap(),
        time_point("2027-04-20-17-00-00")
            .unwrap()
            .to_unix_timestamp()
            .unwrap()
    );
}

#[test]
fn convert_to_tai_matches_direct_conversion() {
    let local = time_point("2017-01-01-05-00-00")
        .unwrap()
        .with_zone(TimeZone::fixed(-5, 0).unwrap());

    let utc = local.to_utc().unwrap();
    let via_zone = local.convert_to(TimeZone::TAI).unwrap();
    let direct = date_time::tai::utc_to_tai(&utc).unwrap();

    assert_eq!(via_zone, direct);
    assert_eq!(via_zone.zone, TimeZone::TAI);
}

#[test]
fn tai_offset_rejects_fixed_zone() {
    let local = time_point("2027-04-20-12-00-00")
        .unwrap()
        .with_zone(TimeZone::fixed(-5, 0).unwrap());

    assert!(date_time::tai::tai_utc_offset(&local).is_err());
}

#[test]
fn convert_to_unix_preserves_fields_as_metadata() {
    let utc = time_point("2027-04-20-12-00-00").unwrap();

    let unix = utc.convert_to(TimeZone::Unix).unwrap();

    assert_eq!(unix.zone, TimeZone::Unix);
    assert_eq!(unix.hour, 12);
    assert_eq!(
        unix,
        time_point("2027-04-20-12-00-00")
            .unwrap()
            .with_zone(TimeZone::Unix)
    );
}
