pub fn is_leap_year(year: &u32) -> bool {
    (*year % 4 == 0 && *year % 100 != 0) || (*year % 400 == 0)
}

pub fn days_in_month(year: &u32, month: &u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}

pub fn valid_date(year: &u32, month: Option<&u32>, day: Option<&u32>) -> bool {
    if let Some(month) = month {
        if *month == 0 || *month > 12 {
            return false;
        }
    }
    if let Some(day) = day {
        if *day == 0 || *day > days_in_month(year, month.unwrap_or(&1)) {
            return false;
        }
    }
    true
}