use std::path::PathBuf;
use std::str::FromStr;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::ParseError;
use chrono::TimeZone as _;
use chrono::Utc;

use super::Config;
use super::Error;
use super::io;
use super::layout::StorageLayout;

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}

pub fn clock() -> NaiveTime {
    Local::now().time()
}

pub fn parse_time(s: &str) -> Result<NaiveTime, ParseError> {
    if let Ok(t) = NaiveTime::from_str(s) {
        return Ok(t);
    }

    NaiveTime::parse_from_str(s, "%H:%M")
}

pub fn parse_date(s: &str) -> Result<NaiveDate, ParseError> {
    if let Ok(t) = NaiveDate::from_str(s) {
        return Ok(t);
    }

    let today = Local::now().date_naive();
    let partial = NaiveDate::parse_from_str(s, "%d.%m")?;

    let date = NaiveDate::from_ymd_opt(today.year(), partial.month(), partial.day()).unwrap();

    Ok(date)
}

pub fn from_naive(date: &NaiveDate, time: &NaiveTime) -> DateTime<Utc> {
    let naive = date.and_time(*time);
    let local_dt = Local
        .from_local_datetime(&naive)
        .single()
        .expect("Ambiguous or invalid local datetime (DST issue?)");
    local_dt.with_timezone(&Utc)
}

pub fn validate_stream(s: &str) -> Result<String, String> {
    let valid = s
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

    if valid {
        Ok(s.to_string())
    } else {
        Err("Stream names can only contain letters, numbers, '-' or '_'".to_string())
    }
}

pub fn parse_existing_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.exists() {
        return Err(format!("'{}' does not exist", s));
    }
    if !path.is_file() {
        return Err(format!("'{}' is not a file", s));
    }
    Ok(path)
}

pub fn storage(preference: Option<PathBuf>) -> Result<io::Storage, Error> {
    let storage_path = io::Settings::storage_path()?;
    let layout = StorageLayout::coalesce(preference, Some(storage_path));

    Ok(io::Storage::new(layout))
}

pub fn config(
    preference: Option<PathBuf>,
    storage: &io::Storage,
    stream: &Option<String>,
) -> Result<Config, Error> {
    let preferenced_config = match preference {
        Some(path) => io::json::read(&path)?,
        None => None,
    };

    let config = Config::coalesce(
        preferenced_config,
        io::Settings::config_file()?,
        storage.config_file(stream)?,
    );

    Ok(config)
}
