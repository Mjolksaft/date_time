pub use date_time::leap_second::parse_leap_seconds;
pub use date_time::util::{parse_month};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_leap_second_dates_from_sample_data() {
        let data = "
# comment
2272060800      10      # 1 Jan 1972
3692217600      37      # 1 Jan 2017
";

        let result = parse_leap_seconds(data);

        assert!(result.contains(&(1971, 12, 31)));
        assert!(result.contains(&(2016, 12, 31)));
    }

    #[test]
    fn ignores_empty_lines_and_comments() {
        let data = "
# this is a comment

not a valid line
";

        let result = parse_leap_seconds(data);

        assert!(result.is_empty());
    }

    #[test]
    fn ignores_lines_without_valid_comment_date() {
        let data = "
3692217600      37
3692217600      37      # bad data here
3692217600      37      # 1 Foo 2017
";

        let result = parse_leap_seconds(data);

        assert!(result.is_empty());
    }

    #[test]
    fn parses_multiple_valid_leap_seconds() {
        let data = "
2272060800      10      # 1 Jan 1972
2287785600      11      # 1 Jul 1972
3692217600      37      # 1 Jan 2017
";

        let result = parse_leap_seconds(data);

        assert_eq!(result.len(), 3);
        assert!(result.contains(&(1971, 12, 31)));
        assert!(result.contains(&(1972, 6, 30)));
        assert!(result.contains(&(2016, 12, 31)));
    }

    #[test]
    fn parse_month_accepts_known_months() {
        assert_eq!(parse_month("Jan"), Some(1));
        assert_eq!(parse_month("Dec"), Some(12));
    }

    #[test]
    fn parse_month_rejects_unknown_month() {
        assert_eq!(parse_month("Foo"), None);
    }
}