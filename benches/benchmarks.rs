use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use date_time::duration::Duration;
use date_time::interval::{interval, to_interval};
use date_time::period::Period;
use date_time::precision::Precision;
use date_time::time_point::time_point;
use date_time::time_zone::TimeZone;
use date_time::uncertainty::Uncertainty;

fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse full timestamp", |b| {
        b.iter(|| time_point("2027-04-20-13-45-30-250"))
    });
    c.bench_function("parse date only", |b| b.iter(|| time_point("2027-04-20")));
}

fn bench_construct(c: &mut Criterion) {
    c.bench_function("TimePoint::new", |b| {
        b.iter(|| {
            date_time::time_point::TimePoint::new(
                2027,
                Some(4),
                Some(20),
                Some(13),
                Some(45),
                Some(30),
                Some(250),
            )
        })
    });
    c.bench_function("boundary_key encode", |b| {
        let t = time_point("2027-04-20-13-45-30-250").unwrap();
        b.iter(|| t.boundary_key())
    });
}

fn bench_arithmetic(c: &mut Criterion) {
    let t = time_point("2027-04-20-13-45-30").unwrap();
    let d = Duration::from_hours(1);
    let p_days = Period::from_days(30);
    let p_years = Period::from_years(1);

    c.bench_function("add_seconds 3600", |b| b.iter(|| t.add_seconds(3600)));
    c.bench_function("add_seconds_fast 3600", |b| {
        b.iter(|| t.add_seconds_fast(3600))
    });
    c.bench_function("sub_seconds 3600", |b| b.iter(|| t.sub_seconds(3600)));
    c.bench_function("add_duration 1h", |b| b.iter(|| t.add_duration(d)));
    c.bench_function("sub_duration 1h", |b| b.iter(|| t.sub_duration(d)));
    c.bench_function("add_period 30d", |b| b.iter(|| t.add_period(p_days)));
    c.bench_function("add_period 1y", |b| b.iter(|| t.add_period(p_years)));
    c.bench_function("duration_since", |b| {
        b.iter(|| t.duration_since(&t.sub_seconds(90)))
    });
}

fn bench_intervals(c: &mut Criterion) {
    let a = time_point("2027-04-20-00-00-00").unwrap();
    let z = time_point("2027-04-21-00-00-00").unwrap();
    let c2 = time_point("2027-04-22-00-00-00").unwrap();
    let iv1 = interval(&a, &z).unwrap();
    let iv2 = interval(&z, &c2).unwrap();

    c.bench_function("interval()", |b| b.iter(|| interval(&a, &z)));
    c.bench_function("allen_relation", |b| b.iter(|| iv1.allen_relation(&iv2)));
    c.bench_function("to_interval with uncertainty", |b| {
        let p = a.clone().with_uncertainty(Uncertainty::from_seconds(60));
        b.iter_batched(
            || p.clone(),
            |p| to_interval(&p, None),
            BatchSize::SmallInput,
        )
    });
}

fn bench_zones(c: &mut Criterion) {
    let t = time_point("2027-04-20-13-45-30")
        .unwrap()
        .with_zone(TimeZone::fixed(-5, 0).unwrap());
    let target = TimeZone::fixed(5, 30).unwrap();

    c.bench_function("convert_to fixed", |b| b.iter(|| t.convert_to(target)));
    c.bench_function("to_utc", |b| b.iter(|| t.to_utc()));
    c.bench_function("to_unix_timestamp", |b| b.iter(|| t.to_unix_timestamp()));
}

fn bench_unix_tai(c: &mut Criterion) {
    let t = time_point("2027-04-20-13-45-30").unwrap();

    c.bench_function("unix round trip", |b| {
        b.iter(|| {
            let ts = t.to_unix_timestamp().unwrap();
            date_time::time_point::TimePoint::from_unix_timestamp(ts)
        })
    });
    c.bench_function("utc_to_tai", |b| b.iter(|| date_time::tai::utc_to_tai(&t)));
}

fn bench_precision(c: &mut Criterion) {
    let mut p = time_point("2027-04-20-13-45-30").unwrap();
    c.bench_function("precision resize", |b| {
        b.iter(|| {
            p.precision = if p.precision == Precision::Day {
                Precision::Second
            } else {
                Precision::Day
            };
        })
    });
}

criterion_group!(
    benches,
    bench_parse,
    bench_construct,
    bench_arithmetic,
    bench_intervals,
    bench_zones,
    bench_unix_tai,
    bench_precision,
);
criterion_main!(benches);
