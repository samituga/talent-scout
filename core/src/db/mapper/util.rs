use chrono::DateTime;
use sea_orm::prelude::DateTimeWithTimeZone;

pub fn unix_timestamp_to_date_time(unix_timestamp_ms: i64) -> anyhow::Result<DateTimeWithTimeZone> {
    let secs = unix_timestamp_ms / 1000;
    let nsecs = ((unix_timestamp_ms % 1000) * 1_000_000) as u32;
    match DateTime::from_timestamp(secs, nsecs) {
        None => Err(anyhow::anyhow!("Failed to parse unix timestamp to DateTime")),
        Some(date_time) => Ok(date_time.fixed_offset()),
    }
}
