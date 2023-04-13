use chrono::{DateTime, Utc};
use std::time::Duration;
use crossterm::style::{StyledContent, Stylize};
use descriptor::Describe;
use number_prefix::NumberPrefix;
use std::fs;

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

pub fn underlined(str: &String) -> DescribedStyledContent {
    str.to_string().underlined().into()
}

pub fn bold_option(opt: &Option<String>) -> DescribedStyledContent {
    opt.clone().unwrap_or("".to_string()).bold().into()
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

pub fn color_status(state: &String) -> DescribedStyledContent {
    match state.as_str() {
        "PENDING" | "QUEUED" | "INITIALIZING" | "FINALIZING" => state.to_string().blue().into(),
        "FAILED" | "ERROR" => state.to_string().red().into(),
        "RUNNING" | "DONE" => state.to_string().green().into(),
        "TIMEOUT" | "INTERRUPTING" | "INTERRUPTED" => state.to_string().yellow().into(),
        _ => state.to_string().blue().into(),
    }
}

#[derive(Debug, Clone)]
pub struct PageQueryParams {
    pub page: Option<u32>,
    pub size: Option<u32>,
}

impl PageQueryParams {
    pub fn enrich_query(&self, query: &Vec<(String, String)>) -> Vec<(String, String)> {
        let mut enriched_query = query.clone();
        if let Some(page) = self.page {
            enriched_query.push(("page".to_string(), page.to_string()))
        }
        if let Some(size) = self.size {
            enriched_query.push(("size".to_string(), size.to_string()))
        }
        enriched_query
    }

    pub fn has_params(&self) -> bool {
        self.page.is_some() || self.size.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct SortQueryParams {
    pub order: Option<String>,
    pub sort: Option<String>,
    pub updated_after: Option<i64>,
    pub updated_before: Option<i64>,
}

impl SortQueryParams {
    pub fn enrich_query(&self, query: &Vec<(String, String)>) -> Vec<(String, String)> {
        let mut enriched_query = query.clone();
        if let Some(order) = &self.order {
            enriched_query.push(("order".to_string(), order.to_string()))
        }
        if let Some(sort) = &self.sort {
            enriched_query.push(("sort".to_string(), sort.to_string()))
        }
        if let Some(updated_after) = self.updated_after {
            enriched_query.push(("updatedAfter".to_string(), updated_after.to_string()))
        }
        if let Some(updated_before) = self.updated_before {
            enriched_query.push(("updatedBefore".to_string(), updated_before.to_string()))
        }
        enriched_query
    }

    pub fn has_params(&self) -> bool {
        self.order.is_some() || self.sort.is_some() || self.updated_after.is_some() || self.updated_before.is_some()
    }
}

pub struct DescribedStyledContent {
    styled_content: StyledContent<String>,
}

impl From<StyledContent<String>> for DescribedStyledContent {
    fn from(styled_content: StyledContent<String>) -> Self {
        DescribedStyledContent {
            styled_content: styled_content,
        }
    }
}

impl Describe for DescribedStyledContent {
    fn to_field(&self, _: &str) -> String {
        self.styled_content.to_string()
    }
    fn default_headers() -> Vec<String> {
        Self::headers()
    }
    fn headers() -> Vec<String> {
        vec![]
    }
    fn header_name(_: &str) -> Option<String> {
        None
    }
}

/// If the given potential_file_path string is a file path that exists, we return the content of the file, if not, return potential_file_path as it is
pub fn file_content_or_string(potential_file_path: String) -> String {
    // If the given string is a path, we put the content of the file, if not we use the string as it is
    return match fs::read_to_string(&potential_file_path) {
        Ok(key) => key,
        Err(_) => potential_file_path.clone(),
    };
}

#[cfg(test)]
pub fn age(date: &DateTime<Utc>) -> String {
    use chrono::TimeZone;
    let now = Utc.with_ymd_and_hms(2021, 7, 8 , 9, 10, 11).unwrap();
    let age = now - date.clone();
    human_duration(&age.num_seconds())
}

#[cfg(not(test))]
pub fn age(date: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let age = now - date.clone();
    human_duration(&age.num_seconds())
}

#[cfg(test)]
pub fn duration(start: &Option<DateTime<Utc>>, end:&Option<DateTime<Utc>>) -> String {
    use chrono::TimeZone;
    let now = Utc.with_ymd_and_hms(2021, 7, 8 , 9, 10, 11).unwrap();
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
pub fn duration(start: &Option<DateTime<Utc>>, end:&Option<DateTime<Utc>>) -> String {
    let now = Utc::now();
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

#[cfg(test)]
pub fn datetime_micro(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    micro: u32,
) -> DateTime<Utc> {
    use chrono::{TimeZone, Timelike};
    Utc.with_ymd_and_hms(year, month, day, hour, min, sec).unwrap().with_nanosecond(micro * 1000).unwrap()
}
