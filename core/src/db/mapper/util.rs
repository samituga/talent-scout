use chrono::DateTime;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use sea_orm::prelude::DateTimeWithTimeZone;

// TODO check if this works
pub fn f32_to_decimal(n: f32) -> Decimal {
    Decimal::from_f32(n).expect("Failed to convert f32 to decimal") // Need to confirm if this ever fails
}

pub fn f64_to_decimal(n: f64) -> Decimal {
    Decimal::from_f64(n).expect("Failed to convert f64 to decimal") // Need to confirm if this ever fails
}

pub fn i32_to_bool(n: i32) -> bool {
    match n {
        x if x < 0 => {
            // TODO log error or warning, this should not be reachable
            eprintln!("Error when converting i32 to bool, {n} is smaller than 0");
            false
        }
        0 => false,
        1 => true,
        x if x > 1 => {
            // TODO log error or warning, this should not be reachable
            eprintln!("Error when converting i32 to bool, {n} is bigger than 1");
            true
        }
        _ => unreachable!(),
    }
}

pub fn unix_timestamp_to_date_time(unix_timestamp_ms: i64) -> anyhow::Result<DateTimeWithTimeZone> {
    let secs = unix_timestamp_ms / 1000;
    let nsecs = ((unix_timestamp_ms % 1000) * 1_000_000) as u32;
    match DateTime::from_timestamp(secs, nsecs) {
        None => Err(anyhow::anyhow!("Failed to parse unix timestamp to DateTime")),
        Some(date_time) => Ok(date_time.fixed_offset()),
    }
}
