pub fn is_leap_year(year: &u32) -> bool {
    (*year % 4 == 0 && *year % 100 != 0) || (*year % 400 == 0)
}

pub fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(&year) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}

pub fn valid_date(year: u32, month: Option<u32>, day: Option<u32>, hour: Option<u32>, minute: Option<u32>, second: Option<u32>) -> Result<(), String> {
    if let Some(month) = month {
        if month == 0 || month > 12 {
            return Err(String::from("Invalid month"));
        }

        if let Some(day) = day {
            if day == 0 || day > days_in_month(year, month) {
                return Err(String::from("Invalid day"));
            }
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
            if second > 59 {
                return Err(String::from("Invalid second"));
            }
        }
    } else if day.is_some() {
        return Err(String::from("Day provided without month"));
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

pub fn previous_day(year: u32, month: u32, day: u32) -> (u32, u32, u32) {
    if day > 1 {
        (year, month, day - 1)
    } else {
        if month == 1 {
            (year - 1, 12, 31)
        } else {
            let prev_month = month - 1;
            let last_day = days_in_month(year, prev_month);
            (year, prev_month, last_day)
        }
    }
}