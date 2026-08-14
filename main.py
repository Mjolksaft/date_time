import calendar as _cal
import datetime as _dt
import math

import streamlit as st
import date_time as dt
from streamlit_autorefresh import st_autorefresh

st.set_page_config(page_title="date_time - Temporal Visual Lab", layout="wide")

st.title("date_time - Temporal Visual Lab")
st.caption(
    "A Rust temporal model, explored visually: zones, intervals, uncertainty, "
    "calendar-aware periods, and leap seconds."
)

ALLEN_COLORS = {
    "before": "#94a3b8",
    "after": "#94a3b8",
    "meets": "#3b82f6",
    "met-by": "#60a5fa",
    "overlaps": "#f97316",
    "overlapped-by": "#fb923c",
    "contains": "#22c55e",
    "during": "#a855f7",
    "starts": "#14b8a6",
    "started-by": "#2dd4bf",
    "finishes": "#ec4899",
    "finished-by": "#f472b6",
    "equal": "#10b981",
}

TV_COLOR = {"True": "#22c55e", "False": "#ef4444", "Unknown": "#f59e0b"}


def svg(inner, width, height):
    return (
        f'<svg width="{width}" height="{height}" '
        'xmlns="http://www.w3.org/2000/svg" role="img">'
        f"{inner}</svg>"
    )


def fmt_time(t):
    return f"{t.hour():02d}:{t.minute():02d}:{t.second():02d}"


def fmt_date(t):
    return f"{t.year():04d}-{t.month():02d}-{t.day():02d}"


def fmt_epoch(ts):
    return _dt.datetime.fromtimestamp(ts, tz=_dt.timezone.utc).strftime("%H:%M:%S")


def chip(text, color):
    return (
        f'<span style="background:{color}1f;color:{color};border:1px solid {color}55;'
        f'border-radius:999px;padding:3px 12px;font-size:14px;font-weight:600;'
        f'margin:3px;display:inline-block;">{text}</span>'
    )


def svg_clock(h, m, s, size=240):
    cx = cy = size / 2
    r = size * 0.44
    parts = [
        f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="#1e293b" '
        f'stroke="#64748b" stroke-width="3"/>'
    ]
    for i in range(12):
        a = math.radians(i * 30 - 90)
        x1, y1 = cx + (r - 7) * math.cos(a), cy + (r - 7) * math.sin(a)
        x2, y2 = cx + r * math.cos(a), cy + r * math.sin(a)
        parts.append(
            f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" '
            f'stroke="#cbd5e1" stroke-width="2.5"/>'
        )

    def hand(angle_deg, length):
        a = math.radians(angle_deg - 90)
        return cx + length * math.cos(a), cy + length * math.sin(a)

    x, y = hand((h % 12) * 30 + m * 0.5, r * 0.5)
    parts.append(
        f'<line x1="{cx}" y1="{cy}" x2="{x:.1f}" y2="{y:.1f}" stroke="#f8fafc" '
        f'stroke-width="6" stroke-linecap="round"/>'
    )
    x, y = hand(m * 6 + s * 0.1, r * 0.72)
    parts.append(
        f'<line x1="{cx}" y1="{cy}" x2="{x:.1f}" y2="{y:.1f}" stroke="#38bdf8" '
        f'stroke-width="4" stroke-linecap="round"/>'
    )
    x, y = hand(s * 6, r * 0.82)
    parts.append(
        f'<line x1="{cx}" y1="{cy}" x2="{x:.1f}" y2="{y:.1f}" stroke="#f43f5e" '
        f'stroke-width="2" stroke-linecap="round"/>'
    )
    parts.append(f'<circle cx="{cx}" cy="{cy}" r="4.5" fill="#f43f5e"/>')
    return svg("".join(parts), size, size)


def svg_counter(h, m, s, sec_color="#e2e8f0", width=520, height=150):
    parts = [
        f'<rect width="{width}" height="{height}" rx="18" fill="#0f172a" '
        f'stroke="#334155" stroke-width="2"/>'
    ]
    parts.append(
        f'<text x="{width / 2}" y="{height / 2 + 24}" text-anchor="middle" '
        f'font-family="Consolas, monospace" font-size="76" font-weight="700" '
        f'fill="#e2e8f0">{h:02d}:{m:02d}:'
        f'<tspan fill="{sec_color}">{s:02d}</tspan></text>'
    )
    return svg("".join(parts), width, height)


def svg_timeline(
    segments,
    lo,
    hi,
    width=980,
    bar_h=34,
    gap=12,
    lo_label="",
    hi_label="",
):
    margin = 46
    top = 34
    usable = width - 2 * margin
    n_bars = sum(1 for s in segments if not s.get("line"))
    height = top + n_bars * (bar_h + gap) + 30

    def x(v):
        return margin + (v - lo) / (hi - lo) * usable

    parts = [
        f'<line x1="{margin}" y1="{top + bar_h}" x2="{width - margin}" '
        f'y2="{top + bar_h}" stroke="#475569" stroke-width="2"/>'
    ]
    for v, lab in ((lo, lo_label), (hi, hi_label)):
        px = x(v)
        parts.append(
            f'<line x1="{px:.1f}" y1="{top + bar_h - 6}" x2="{px:.1f}" '
            f'y2="{top + bar_h + 6}" stroke="#94a3b8" stroke-width="2"/>'
        )
        parts.append(
            f'<text x="{px:.1f}" y="{top + bar_h + 20}" text-anchor="middle" '
            f'font-size="12" fill="#94a3b8">{lab}</text>'
        )

    y = top
    for seg in segments:
        if seg.get("line"):
            px = x(seg["start"])
            parts.append(
                f'<line x1="{px:.1f}" y1="{y}" x2="{px:.1f}" y2="{y + bar_h}" '
                f'stroke="{seg["color"]}" stroke-width="3"/>'
            )
            parts.append(
                f'<circle cx="{px:.1f}" cy="{y + bar_h / 2}" r="5" '
                f'fill="{seg["color"]}"/>'
            )
            if seg.get("label"):
                parts.append(
                    f'<text x="{px:.1f}" y="{y - 6}" text-anchor="middle" '
                    f'font-size="12" fill="{seg["color"]}">{seg["label"]}</text>'
                )
            y += bar_h + gap
            continue

        x1, x2 = x(seg["start"]), x(seg["end"])
        w = max(x2 - x1, 3)
        opacity = seg.get("opacity", 0.85)
        parts.append(
            f'<rect x="{x1:.1f}" y="{y}" width="{w:.1f}" height="{bar_h}" '
            f'rx="9" fill="{seg["color"]}" opacity="{opacity}"/>'
        )
        if seg.get("label"):
            parts.append(
                f'<text x="{x1 + 10:.1f}" y="{y + bar_h / 2 + 5:.1f}" '
                f'font-size="13" font-weight="700" fill="#ffffff">{seg["label"]}</text>'
            )
        y += bar_h + gap

    return svg("".join(parts), width, height)


def svg_calendar(year, month, highlight=None, width=420, cell=46, title=None):
    highlight = highlight or {}
    cw, ch = cell, cell
    header_h, title_h = 26, (30 if title else 0)
    weeks = _cal.Calendar(firstweekday=0).monthdayscalendar(year, month)
    height = title_h + header_h + len(weeks) * ch + 4
    parts = []
    if title:
        parts.append(
            f'<text x="{width / 2}" y="20" text-anchor="middle" font-size="15" '
            f'font-weight="700" fill="#e2e8f0">{title}</text>'
        )
    y = title_h
    for i, d in enumerate(["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"]):
        parts.append(
            f'<text x="{i * cw + cw / 2}" y="{y + 17}" text-anchor="middle" '
            f'font-size="12" fill="#94a3b8">{d}</text>'
        )
    y += header_h
    for week in weeks:
        for i, d in enumerate(week):
            if d == 0:
                continue
            px, py = i * cw, y
            if d in highlight:
                parts.append(
                    f'<rect x="{px + 1}" y="{py + 1}" width="{cw - 2}" '
                    f'height="{ch - 2}" rx="9" fill="{highlight[d]}"/>'
                )
                parts.append(
                    f'<text x="{px + cw / 2}" y="{py + ch / 2 + 5}" text-anchor="middle" '
                    f'font-size="14" font-weight="700" fill="#0f172a">{d}</text>'
                )
            else:
                parts.append(
                    f'<rect x="{px + 1}" y="{py + 1}" width="{cw - 2}" '
                    f'height="{ch - 2}" rx="9" fill="#1e293b" stroke="#334155" '
                    f'stroke-width="1"/>'
                )
                parts.append(
                    f'<text x="{px + cw / 2}" y="{py + ch / 2 + 5}" text-anchor="middle" '
                    f'font-size="13" fill="#cbd5e1">{d}</text>'
                )
        y += ch
    return svg("".join(parts), width, y + 4)


def zone_card(name, offset_label, time_str, date_str, same_day):
    border = "#334155" if same_day else "#f59e0b"
    if same_day:
        badge = (
            '<span style="background:#22c55e1f;color:#22c55e;border-radius:999px;'
            'padding:1px 10px;font-size:12px;">same day as UTC</span>'
        )
    else:
        badge = (
            '<span style="background:#f973161f;color:#f97316;border-radius:999px;'
            'padding:1px 10px;font-size:12px;">different day</span>'
        )
    return (
        f'<div style="border:1px solid {border};border-radius:14px;padding:14px;'
        f'text-align:center;background:#0f172a;">'
        f'<div style="color:#94a3b8;font-size:13px;margin-bottom:2px;">{name}</div>'
        f'<div style="font-size:13px;color:#64748b;margin-bottom:8px;">{offset_label}</div>'
        f'<div style="font-size:25px;font-weight:700;color:#f8fafc;'
        f'font-family:Consolas,monospace;">{time_str}</div>'
        f'<div style="color:#cbd5e1;font-size:13px;margin-top:4px;">{date_str}</div>'
        f'<div style="margin-top:10px;">{badge}</div></div>'
    )


tab_world, tab_interval, tab_uncert, tab_calendar, tab_leap = st.tabs(
    ["World Clock", "Interval Explorer", "Uncertainty", "Calendar Math", "Leap Second"]
)

# ---------------------------------------------------------------- World Clock
with tab_world:
    live = st.toggle("Live UTC clock", value=True, key="live_toggle")

    if live:
        st_autorefresh(interval=1000, key="wc_refresh")
        clock = dt.PyTimePoint.now_utc()
        st.session_state.clock = clock
    else:
        if "clock" not in st.session_state:
            st.session_state.clock = dt.PyTimePoint.now_utc()
        clock = st.session_state.clock

        c1, c2, c3, c4, c5, c6 = st.columns(6)
        with c1:
            if st.button("+1 sec"):
                st.session_state.clock = clock.add_seconds(1)
        with c2:
            if st.button("+1 min"):
                st.session_state.clock = clock.add_minutes(1)
        with c3:
            if st.button("+1 hour"):
                st.session_state.clock = clock.add_hours(1)
        with c4:
            if st.button("-1 sec"):
                st.session_state.clock = clock.sub_seconds(1)
        with c5:
            if st.button("-1 min"):
                st.session_state.clock = clock.sub_minutes(1)
        with c6:
            if st.button("-1 hour"):
                st.session_state.clock = clock.sub_hours(1)
        clock = st.session_state.clock

    left, right = st.columns([1.1, 1])
    with left:
        st.markdown(svg_clock(clock.hour(), clock.minute(), clock.second()), unsafe_allow_html=True)
    with right:
        st.markdown(
            f'<div style="font-size:58px;font-weight:700;color:#f8fafc;'
            f'font-family:Consolas,monospace;">{fmt_time(clock)}</div>',
            unsafe_allow_html=True,
        )
        st.markdown(
            f'<div style="color:#94a3b8;font-size:16px;">{fmt_date(clock)} '
            f'&middot; zone {clock.zone_label()} &middot; precision {clock.precision_label()}</div>',
            unsafe_allow_html=True,
        )
        st.caption("Same instant, different zones below - the date changes while the instant does not.")

    utc = clock.to_utc()
    utc_date = fmt_date(utc)

    zones = [
        ("UTC", dt.PyTimeZone.utc),
        ("New York (winter)", dt.PyTimeZone.fixed(-5, 0)),
        ("India", dt.PyTimeZone.fixed(5, 30)),
        ("Kiribati", dt.PyTimeZone.fixed(14, 0)),
    ]

    cols = st.columns(len(zones))
    for col, (name, zone) in zip(cols, zones):
        local = clock.convert_to(zone)
        with col:
            st.markdown(
                zone_card(
                    name,
                    zone.offset_label(),
                    fmt_time(local),
                    fmt_date(local),
                    fmt_date(local) == utc_date,
                ),
                unsafe_allow_html=True,
            )

# ------------------------------------------------------- Interval Explorer
with tab_interval:
    st.subheader("Drag the intervals and watch the relation")
    base = dt.PyTimePoint.parse("2027-04-20-00-00-00")
    hi = 3600

    sa, ea = st.slider("Interval A - start / end (seconds after base)", 0, hi, (300, 1500), key="sl_a")
    sb, eb = st.slider("Interval B - start / end (seconds after base)", 0, hi, (2000, 3100), key="sl_b")

    def ordered(s, e):
        if s == e:
            e = min(s + 1, hi)
        return (s, e) if s < e else (e, s)

    as_, ae = ordered(sa, ea)
    bs, be = ordered(sb, eb)

    a_lo, a_hi = base.add_seconds(as_), base.add_seconds(ae)
    b_lo, b_hi = base.add_seconds(bs), base.add_seconds(be)

    iv_a = dt.PyInterval.interval(a_lo, a_hi)
    iv_b = dt.PyInterval.interval(b_lo, b_hi)

    rel = iv_a.allen_relation(iv_b)
    rel_color = ALLEN_COLORS.get(rel, "#94a3b8")

    st.markdown(
        f'<div style="font-size:22px;font-weight:700;margin:6px 0;">'
        f'A {chip(rel, rel_color)} B</div>',
        unsafe_allow_html=True,
    )

    st.markdown(
        svg_timeline(
            [
                {"start": as_, "end": ae, "color": "#3b82f6", "label": "A"},
                {"start": bs, "end": be, "color": "#f97316", "label": "B"},
            ],
            0,
            hi,
            lo_label="base 2027-04-20 00:00",
            hi_label="+1h",
        ),
        unsafe_allow_html=True,
    )

    preds = [
        ("A before B", iv_a.before(iv_b)),
        ("A after B", iv_a.after(iv_b)),
        ("A equals B", iv_a.equals(iv_b)),
        ("A contains B", iv_a.contains(iv_b)),
        ("A overlaps B", iv_a.overlaps(iv_b)),
    ]
    st.markdown(
        " ".join(chip(f"{name} = {v}", TV_COLOR[v]) for name, v in preds),
        unsafe_allow_html=True,
    )

    st.caption(
        "Three-valued logic: True / False / Unknown. The five predicates are "
        "consistent with the 13-way Allen classification below."
    )
    legend = " ".join(chip(name, ALLEN_COLORS[name]) for name in ALLEN_COLORS)
    st.markdown(legend, unsafe_allow_html=True)

# -------------------------------------------------------------- Uncertainty
with tab_uncert:
    st.subheader("Uncertainty spreads a point into an interval")
    base = dt.PyTimePoint.parse("2027-04-20-12-00-00")
    u = st.slider("Uncertainty (seconds)", 0, 1800, 300, key="u")
    d = st.slider("Advance d seconds", 0, 3600, 0, key="d")

    p = base.with_uncertainty(dt.PyUncertainty.from_seconds(u))
    t0 = base.to_unix_timestamp()
    lo = t0 - u - 90
    hi = t0 + d + u + 90

    st.markdown(
        svg_timeline(
            [
                {"start": t0 - u, "end": t0 + u, "color": "#38bdf8", "label": "t ± u", "opacity": 0.5},
                {"start": t0, "end": t0, "color": "#38bdf8", "label": "t", "line": True},
                {"start": t0 + d - u, "end": t0 + d + u, "color": "#f97316", "label": "(t + d) ± u", "opacity": 0.5},
                {"start": t0 + d, "end": t0 + d, "color": "#f97316", "label": "t + d", "line": True},
            ],
            lo,
            hi,
            lo_label=fmt_epoch(lo),
            hi_label=fmt_epoch(hi),
        ),
        unsafe_allow_html=True,
    )

    st.markdown(
        f'<div style="text-align:center;font-size:17px;font-weight:600;color:#e2e8f0;">'
        f'(t &plusmn; {u}s) + {d}s = (t + {d}s) &plusmn; {u}s</div>',
        unsafe_allow_html=True,
    )

    iv = dt.PyInterval.to_interval(p, None)
    c1, c2 = st.columns(2)
    c1.metric("interval lower_key", iv.lower_key())
    c2.metric("interval upper_key", iv.upper_key())
    st.caption("to_interval(t, u) builds [t - u, t + u) directly; add_seconds shifts the whole band.")

# -------------------------------------------------------------- Calendar Math
with tab_calendar:
    st.subheader("Periods are calendar-aware, not just 24-hour chunks")

    c1, c2, c3 = st.columns(3)
    with c1:
        start_date = st.date_input("Start date", value=_dt.date(2024, 2, 29), key="start_date")
    with c2:
        years = st.number_input("Years", 0, 50, 1, key="p_years")
    with c3:
        months = st.number_input("Months", 0, 60, 0, key="p_months")
    days = st.number_input("Days", 0, 90, 0, key="p_days")

    start = dt.PyTimePoint.parse(
        f"{start_date.year:04d}-{start_date.month:02d}-{start_date.day:02d}"
    )
    result = start.add_period(
        dt.PyPeriod(years=int(years), months=int(months), days=int(days))
    )
    ry, rm, rd = result.year(), result.month(), result.day()

    arrow_col = '<div style="text-align:center;font-size:44px;color:#94a3b8;padding-top:70px;">&#8594;</div>'

    left, mid, right = st.columns([1, 0.2, 1])
    with left:
        st.markdown(
            svg_calendar(
                start_date.year, start_date.month, {start_date.day: "#3b82f6"},
                title=f"Start {fmt_date(start)}",
            ),
            unsafe_allow_html=True,
        )
    with mid:
        st.markdown(arrow_col, unsafe_allow_html=True)
    with right:
        st.markdown(
            svg_calendar(
                ry, rm, {rd: "#22c55e"},
                title=f"Result {fmt_date(result)}",
            ),
            unsafe_allow_html=True,
        )

    if (ry, rm) != (start_date.year, start_date.month):
        st.caption("The result crossed into another month. Note how the day is clamped "
                   "to the target month when it does not exist there.")

    st.markdown("#### Clamping: impossible dates snap to the last real day")
    l1, lm, l2 = st.columns([1, 0.2, 1])
    with l1:
        r1 = dt.PyTimePoint.parse("2024-02-29").add_period(dt.PyPeriod(years=1))
        st.markdown(
            svg_calendar(2024, 2, {29: "#3b82f6"}, title="2024-02-29 + 1y (leap)"),
            unsafe_allow_html=True,
        )
    with lm:
        st.markdown(arrow_col, unsafe_allow_html=True)
    with l2:
        st.markdown(
            svg_calendar(
                2025, 2, {r1.day(): "#22c55e"},
                title=f"&rarr; {fmt_date(r1)} (Feb 2025 has 28 days)",
            ),
            unsafe_allow_html=True,
        )

    r2m, r2a, r2b = st.columns([1, 0.2, 1])
    with r2m:
        r2 = dt.PyTimePoint.parse("2027-01-31").add_period(dt.PyPeriod(months=1))
        st.markdown(
            svg_calendar(2027, 1, {31: "#3b82f6"}, title="2027-01-31 + 1m"),
            unsafe_allow_html=True,
        )
    with r2a:
        st.markdown(arrow_col, unsafe_allow_html=True)
    with r2b:
        st.markdown(
            svg_calendar(
                2027, 2, {r2.day(): "#22c55e"},
                title=f"&rarr; {fmt_date(r2)} (clamped)",
            ),
            unsafe_allow_html=True,
        )

# --------------------------------------------------------------- Leap Second
with tab_leap:
    st.subheader("The clock that hits :60")
    leap0 = dt.PyTimePoint.parse("2016-12-31-23-59-57")
    n = st.slider("Elapsed seconds from 2016-12-31 23:59:57", 0, 4, 2, key="leap_n")

    point = leap0.add_seconds(n)
    sec = point.second()
    sec_color = "#f43f5e" if sec == 60 else "#e2e8f0"

    st.markdown(
        svg_counter(point.hour(), point.minute(), sec, sec_color),
        unsafe_allow_html=True,
    )

    seq = [leap0.add_seconds(i) for i in range(5)]
    seq_html = " ".join(
        chip(fmt_time(p), "#f43f5e" if p.second() == 60 else "#94a3b8")
        for p in seq
    )
    st.markdown(seq_html, unsafe_allow_html=True)
    st.caption(
        "At 23:59:60 the minute holds 61 seconds. The same instant is continuous "
        "in TAI, which never displays :60."
    )

    tai = dt.utc_to_tai(point)
    c1, c2 = st.columns(2)
    with c1:
        st.metric("UTC", f"{fmt_date(point)} {fmt_time(point)}")
    with c2:
        st.metric("TAI", f"{fmt_date(tai)} {fmt_time(tai)}")
    st.caption(
        "Leap seconds are UTC-anchored: the leap second is a normal TAI second "
        "absorbed into the TAI-UTC offset."
    )
