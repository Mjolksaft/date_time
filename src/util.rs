pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn days_in_month(year: u32, month: u32) -> Result<u32, String> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
        4 | 6 | 9 | 11 => Ok(30),
        2 => {
            if is_leap_year(year) {
                Ok(29)
            } else {
                Ok(28)
            }
        }
        _ => Err(String::from("Invalid month")),
    }
}

pub fn valid_date(
    year: u32,
    month: Option<u32>,
    day: Option<u32>,
    hour: Option<u32>,
    minute: Option<u32>,
    second: Option<u32>,
    millisecond: Option<u32>,
) -> Result<(), String> {
    if let Some(month) = month {
        if month == 0 || month > 12 {
            return Err(String::from("Invalid month"));
        }

        if let Some(day) = day {
            let max_day = days_in_month(year, month)?;

            if day == 0 || day > max_day {
                return Err(String::from("Invalid day"));
            }
        }
    } else if day.is_some() {
        return Err(String::from("Day provided without month"));
    }

    if hour.is_some() && day.is_none() {
        return Err(String::from("Hour provided without day"));
    }

    if minute.is_some() && hour.is_none() {
        return Err(String::from("Minute provided without hour"));
    }

    if second.is_some() && minute.is_none() {
        return Err(String::from("Second provided without minute"));
    }

    if millisecond.is_some() && second.is_none() {
        return Err(String::from("Millisecond provided without second"));
    }

    if let Some(hour) = hour {
        if hour > 23 {
            return Err(String::from("Invalid hour"));
        }
    }

    if let Some(minute) = minute {
        if minute > 59 {
            return Err(String::from("Invalid minute"));
        }
    }

    if let Some(second) = second {
        if second > 60 {
            return Err(String::from("Invalid second"));
        }
    }

    if let Some(millisecond) = millisecond {
        if millisecond > 999 {
            return Err(String::from("Invalid millisecond"));
        }
    }

    Ok(())
}

pub fn parse_month(m: &str) -> Option<u32> {
    match m {
        "Jan" => Some(1),
        "Feb" => Some(2),
        "Mar" => Some(3),
        "Apr" => Some(4),
        "May" => Some(5),
        "Jun" => Some(6),
        "Jul" => Some(7),
        "Aug" => Some(8),
        "Sep" => Some(9),
        "Oct" => Some(10),
        "Nov" => Some(11),
        "Dec" => Some(12),
        _ => None,
    }
}

pub fn previous_day(year: u32, month: u32, day: u32) -> Result<(u32, u32, u32), String> {
    if month == 0 || month > 12 {
        return Err(String::from("Invalid month"));
    }

    let max_day = days_in_month(year, month)?;

    if day == 0 || day > max_day {
        return Err(String::from("Invalid day"));
    }

    if day > 1 {
        return Ok((year, month, day - 1));
    }

    if month == 1 {
        return Ok((year - 1, 12, 31));
    }

    let previous_month = month - 1;
    let last_day = days_in_month(year, previous_month)?;

    Ok((year, previous_month, last_day))
}