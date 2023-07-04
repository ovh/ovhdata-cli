use chrono::{DateTime, Utc};
use number_prefix::NumberPrefix;
use std::fs;
use std::time::Duration;

pub fn human_bytes(size: &usize) -> String {
    match NumberPrefix::binary(*size as f64) {
        NumberPrefix::Standalone(n) => format!("{} B", n),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.1} {}B", n, prefix),
    }
}

pub fn human_count(count: &usize) -> String {
    match NumberPrefix::binary(*count as f64) {
        NumberPrefix::Standalone(n) => format!("{}", n),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.1} {}", n, prefix),
    }
}

pub fn human_bits_speed(size: &usize) -> String {
    match NumberPrefix::decimal(*size as f64) {
        NumberPrefix::Standalone(n) => format!("{} bps", n),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.1} {}bps", n, prefix),
    }
}

pub fn human_duration(sec: &i64) -> String {
    let duration = &Duration::from_secs(*sec as u64);

    let seconds = duration.as_secs();
    let (number, unit) = if seconds < 2 * 60 {
        // Less than 2 minutes
        (seconds, "s")
    } else if seconds < 2 * 3600 {
        // Less than 2 hours
        (seconds / 60, "m")
    } else if seconds < 2 * 24 * 3600 {
        // Less than 2 days
        (seconds / 3600, "h")
    } else {
        // More than 2 days
        (seconds / (24 * 3600), "d")
    };

    format!("{}{}", number, unit)
}

/// If the given potential_file_path string is a file path that exists, we return the content of the file, if not, return potential_file_path as it is
pub fn file_content_or_string(potential_file_path: String) -> String {
    // If the given string is a path, we put the content of the file, if not we use the string as it is
    match fs::read_to_string(&potential_file_path) {
        Ok(key) => key,
        Err(_) => potential_file_path,
    }
}

#[cfg(test)]
pub fn age(date: &DateTime<Utc>) -> String {
    use chrono::TimeZone;
    let now = Utc.with_ymd_and_hms(2021, 7, 8, 9, 10, 11).unwrap();
    let age = now - date.clone();
    human_duration(&age.num_seconds())
}

#[cfg(not(test))]
pub fn age(date: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let age = now - *date;
    human_duration(&age.num_seconds())
}

#[cfg(test)]
pub fn duration(start: &Option<DateTime<Utc>>, end: &Option<DateTime<Utc>>) -> String {
    use chrono::TimeZone;
    let now = Utc.with_ymd_and_hms(2021, 7, 8, 9, 10, 11).unwrap();
    if start.is_none() {
        "not started".to_string()
    } else {
        let start_date = start.unwrap();
        if end.is_none() {
            let duration = now - start_date.clone();
            human_duration(&duration.num_seconds())
        } else {
            let end_date = end.unwrap();
            let duration = end_date.clone() - start_date.clone();
            human_duration(&duration.num_seconds())
        }
    }
}

#[cfg(not(test))]
pub fn duration(start: &Option<DateTime<Utc>>, end: &Option<DateTime<Utc>>) -> String {
    let now = Utc::now();
    if start.is_none() {
        "not started".to_string()
    } else {
        let start_date = start.unwrap();
        if end.is_none() {
            let duration = now - start_date;
            human_duration(&duration.num_seconds())
        } else {
            let end_date = end.unwrap();
            let duration = end_date - start_date;
            human_duration(&duration.num_seconds())
        }
    }
}

#[cfg(test)]
pub fn datetime_micro(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32, micro: u32) -> DateTime<Utc> {
    use chrono::{TimeZone, Timelike};
    Utc.with_ymd_and_hms(year, month, day, hour, min, sec)
        .unwrap()
        .with_nanosecond(micro * 1000)
        .unwrap()
}
