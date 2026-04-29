use reqwest::blocking::get;
use std::{error::Error};
use std::collections::HashSet;
use std::fs;
use crate::util::{parse_month, previous_day};

pub fn fetch_leap_seconds() -> Result<String, Box<dyn Error>> {
    let url = "https://hpiers.obspm.fr/iers/bul/bulc/ntp/leap-seconds.list";

    let response = get(url)?;
    let text = response.text()?;

    Ok(text)
}

pub fn load_from_file() -> std::io::Result<String> {
    fs::read_to_string("leap-seconds.list")
}

pub fn save_to_file(data: &str) -> std::io::Result<()> {
    fs::write("leap-seconds.list", data)
}

pub fn get_leap_seconds_data() -> HashSet<(u32, u32, u32)> {
    match load_from_file() {
        Ok(data) => parse_leap_seconds(&data),
        Err(_) => {
            let data = fetch_leap_seconds().expect("Failed to fetch");
            save_to_file(&data).expect("Failed to save");
            parse_leap_seconds(&data)
        }
    }
}

pub fn parse_leap_seconds(data: &str) -> HashSet<(u32, u32, u32)> {
    let mut set = HashSet::new();

    for line in data.lines() {
        let line = line.trim();

        // skip comments or empty lines
        if line.is_empty() || !line.chars().next().unwrap_or(' ').is_digit(10) {
            continue;
        }

        // split comment part
        if let Some(comment) = line.split('#').nth(1) {
            let parts: Vec<&str> = comment.trim().split_whitespace().collect();

            if parts.len() == 3 {
                let day = parts[0].parse::<u32>().ok();
                let month = parse_month(parts[1]);
                let year = parts[2].parse::<u32>().ok();

                if let (Some(d), Some(m), Some(y)) = (day, month, year) {
                    let (ly, lm, ld) = previous_day(y, m, d);
                    set.insert((ly, lm, ld));
                }
            }
        }
    }

    set
}