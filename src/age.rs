use chrono::{DateTime, Utc, FixedOffset};
use chrono::format::ParseError;

pub fn readable_age_from_string(date_time: &str) -> Result<String, ParseError> {
    let date_time = DateTime::parse_from_rfc3339(date_time).expect("Failed to parse");
    let age = readable_age(date_time).expect("Could not get readable age");
    return Ok(age);
}

pub fn readable_age(date_time: DateTime<FixedOffset>) -> Result<String, ParseError> {
  let now = Utc::now();
  let age = now.signed_duration_since(date_time);
  let seconds = age.num_seconds();
  let age_str = seconds_to_age(seconds);
  return Ok(age_str)
}

fn seconds_to_age(seconds: i64) -> String {
  let years = seconds / (60 * 60 * 24 * 365);
  let months = seconds / (60 * 60 * 24 * 31);
  let weeks = seconds / (60 * 60 * 24 * 7);
  let days = seconds / (60 * 60 * 24);
  let hours = (seconds % (60 * 60 * 24)) / (60 * 60);
  let minutes = (seconds % (60 * 60)) / 60;
  if years > 0 {
    return format_age(years, "year");
  }
  if months > 0 {
    return format_age(months, "month");
  }
  if weeks > 0 {
    return format_age(weeks, "week");
  }
  if days > 0 {
    return format_age(days, "day");
  }
  if hours > 0 {
    return format_age(hours, "hour");
  }
  if minutes > 0 {
    return format_age(minutes, "minute");
  }
  return "just now".to_string();
}

fn format_age(n: i64, s: &str) -> String {
  if n == 1 {
    return format!("{} {} ago", n, s);
  }
  return format!("{} {}s ago", n, s);
}