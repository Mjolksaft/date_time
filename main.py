import streamlit as st
import date_time
from streamlit_autorefresh import st_autorefresh

st.set_page_config(page_title="Temporal Model", layout="wide")

st.title("Temporal Model")

if "clock" not in st.session_state:
    st.session_state.clock = date_time.PyTimePoint.now_utc()

if "manual_time" not in st.session_state:
    st.session_state.manual_time = "2016-12-31-23-59-59"


st.header("Clock")
live_clock = st.toggle("Live UTC Clock", value=True)

if live_clock:
    st_autorefresh(interval=1000, key="clock_refresh")
    current_clock = date_time.PyTimePoint.now_utc()
else:
    if "clock" not in st.session_state:
        st.session_state.clock = date_time.PyTimePoint.now_utc()

    current_clock = st.session_state.clock

col1, col2 = st.columns([2, 1])

with col1:
    st.subheader(str(current_clock))

with col2:
    if st.button("Reset to now UTC"):
        st.session_state.clock = date_time.PyTimePoint.now_utc()

if not live_clock:
    c1, c2, c3, c4, c5, c6 = st.columns(6)

    with c1:
        if st.button("+1 sec"):
            st.session_state.clock = st.session_state.clock.add_seconds(1)

    with c2:
        if st.button("+1 min"):
            st.session_state.clock = st.session_state.clock.add_minutes(1)

    with c3:
        if st.button("+1 hour"):
            st.session_state.clock = st.session_state.clock.add_hours(1)

    with c4:
        if st.button("-1 sec"):
            st.session_state.clock = st.session_state.clock.sub_seconds(1)

    with c5:
        if st.button("-1 min"):
            st.session_state.clock = st.session_state.clock.sub_minutes(1)

    with c6:
        if st.button("-1 hour"):
            st.session_state.clock = st.session_state.clock.sub_hours(1)

st.divider()

st.header("Manual TimePoint")

time_input = st.text_input(
    "Enter TimePoint",
    value=st.session_state.manual_time,
)

try:
    t = date_time.PyTimePoint.parse(time_input)
    st.success("Parsed successfully")
    st.code(str(t))

    m1, m2, m3, m4 = st.columns(4)

    with m1:
        st.write("+1 second")
        st.code(str(t.add_seconds(1)))

    with m2:
        st.write("+10 seconds")
        st.code(str(t.add_seconds(10)))

    with m3:
        st.write("-1 second")
        st.code(str(t.sub_seconds(1)))

    with m4:
        try:
            st.write("Unix timestamp")
            st.code(str(t.to_unix_timestamp()))
        except Exception as e:
            st.warning(str(e))

except Exception as e:
    st.error(str(e))


st.divider()

st.header("Leap Second Demo")

leap = date_time.PyTimePoint.parse("2016-12-31-23-59-59")

st.write("Starting at:")
st.code(str(leap))

l1, l2, l3 = st.columns(3)

with l1:
    st.write("+0 seconds")
    st.code(str(leap))

with l2:
    st.write("+1 second")
    st.code(str(leap.add_seconds(1)))

with l3:
    st.write("+2 seconds")
    st.code(str(leap.add_seconds(2)))


st.divider()

st.header("Interval / Relation Demo")

r1, r2 = st.columns(2)

with r1:
    a_input = st.text_input("TimePoint A", "2027-04")

with r2:
    b_input = st.text_input("TimePoint B", "2027-04-20")

try:
    a = date_time.PyTimePoint.parse(a_input)
    b = date_time.PyTimePoint.parse(b_input)

    st.write("A:")
    st.code(str(a))

    st.write("B:")
    st.code(str(b))

    rel1, rel2, rel3, rel4, rel5 = st.columns(5)

    with rel1:
        st.metric("A before B", a.before(b))

    with rel2:
        st.metric("A after B", a.after(b))

    with rel3:
        st.metric("A equals B", a.equals(b))

    with rel4:
        st.metric("A contains B", a.contains(b))

    with rel5:
        st.metric("A overlaps B", a.overlaps(b))

except Exception as e:
    st.error(str(e))