pub mod time_point;
pub mod precision;
pub mod interval;

use precision::Precision;
use time_point::TimePoint;

fn parse_date(input: &str) -> Result<Vec<u32>, String> {
    let parts: Vec<&str> = input.split("-").collect();

    parts
        .iter()
        .map(|x| x.parse::<u32>().map_err(|_| String::from("Invalid number format")))
        .collect()
}

fn date(input: &str) -> Result<TimePoint, String> {
    if input.is_empty() {
        return Err(String::from("No args"));
    }

    let parsed_date: Vec<u32> = parse_date(input)?;

    match parsed_date.len() {
        1 => Ok(TimePoint {
            year: parsed_date[0],
            month: 1,
            day: 1,
            precision: Precision::Year,
        }),
        2 => Ok(TimePoint {
            year: parsed_date[0],
            month: parsed_date[1],
            day: 1,
            precision: Precision::Month,
        }),
        3 => Ok(TimePoint {
            year: parsed_date[0],
            month: parsed_date[1],
            day: parsed_date[2],
            precision: Precision::Day,
        }),
        _ => Err(String::from("Invalid date format")),
    }
}
#[cfg(test)] // done 
mod parse_tests {

    use super::*;
    #[test]
    fn parses_year_point() {
        let result = date("2027").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 1,
                day: 1,
                precision: Precision::Year,
            }
        );
    }

    #[test]
    fn parses_month_point() {
        let result = date("2027-04").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 1,
                precision: Precision::Month,
            }
        );
    }

    #[test]
    fn parses_day_point() {
        let result = date("2027-04-20").unwrap();

        assert_eq!(
            result,
            TimePoint {
                year: 2027,
                month: 4,
                day: 20,
                precision: Precision::Day,
            }
        );
    }

    #[test]
    fn fails_on_too_many_parts() {
        let result = date("2027-04-10-12");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_empty_input() {
        let result = date("");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_non_numeric_year() {
        let result = date("abcd");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_non_numeric_month() {
        let result = date("2027-ab");
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_non_numeric_day() {
        let result = date("2027-04-xx");
        assert!(result.is_err());
    }
}

#[cfg(test)] // Done 
mod normalization_tests {
    use crate::interval::{Interval, to_interval};

    use super::*;
    
    #[test]
    fn normalizes_year_to_interval() {
        let result = to_interval(&date("2027").unwrap());

        assert_eq!(
            result,
            Interval {
                lower: TimePoint {
                    year: 2027,
                    month: 1,
                    day: 1,
                    precision: Precision::Year,
                },
                upper: TimePoint {
                    year: 2028,
                    month: 1,
                    day: 1,
                    precision: Precision::Year,
                }
            }
        );
    }

    #[test]
    fn normalizes_month_to_interval() {
        let result = to_interval(&date("2027-04").unwrap());

        assert_eq!(
            result,
            Interval {
                lower: TimePoint {
                    year: 2027,
                    month: 4,
                    day: 1,
                    precision: Precision::Month,
                },
                upper: TimePoint {
                    year: 2027,
                    month: 5,
                    day: 1,
                    precision: Precision::Month,
                }
            }
        );
    }

    #[test]
    fn normalizes_day_to_interval() {
        let result = to_interval(&date("2027-04-20").unwrap());

        assert_eq!(
            result,
            Interval {
                lower: TimePoint {
                    year: 2027,
                    month: 4,
                    day: 20,
                    precision: Precision::Day,
                },
                upper: TimePoint {
                    year: 2027,
                    month: 4,
                    day: 21,
                    precision: Precision::Day,
                }
            }
        );
    }

}

#[cfg(test)]
mod equals_tests {
    use super::*;

    #[test]
    fn equals_true_for_same_year() {
        let a = date("2027").unwrap();
        let b = date("2027").unwrap();

        assert!(a.equals(&b));
    }

    #[test]
    fn equals_false_for_year_and_exact_day() {
        let a = date("2027").unwrap();
        let b = date("2027-01-01").unwrap();

        assert!(!a.equals(&b));
    }

    #[test]
    fn equals_true_for_same_month() {
        let a = date("2027-04").unwrap();
        let b = date("2027-04").unwrap();

        assert!(a.equals(&b));
    }
}

#[cfg(test)]
mod before_tests {
    use super::*;

    #[test]
    fn before_true_for_adjacent_days() {
        let a = date("2027-01-01").unwrap();
        let b = date("2027-01-02").unwrap();

        assert!(a.before(&b));
    }

    #[test]
    fn before_true_for_adjacent_months() {
        let a = date("2027-04").unwrap();
        let b = date("2027-05").unwrap();

        assert!(a.before(&b));
    }

    #[test]
    fn before_false_for_year_and_month_inside_it() {
        let a = date("2027").unwrap();
        let b = date("2027-04").unwrap();

        assert!(!a.before(&b));
    }

}



#[cfg(test)]
mod after_tests {
    use super::*;

    #[test]
    fn after_true_for_adjacent_days() {
        let a = date("2027-01-02").unwrap();
        let b = date("2027-01-01").unwrap();

        assert!(a.after(&b));
    }

    #[test]
    fn after_true_for_adjacent_months() {
        let a = date("2027-05").unwrap();
        let b = date("2027-04").unwrap();

        assert!(a.after(&b));
    }

    #[test]
    fn after_false_for_month_and_year_containing_it() {
        let a = date("2027-04").unwrap();
        let b = date("2027").unwrap();

        assert!(!a.after(&b));
    }

    
}
