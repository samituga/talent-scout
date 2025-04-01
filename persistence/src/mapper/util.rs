use rust_decimal::Decimal;

pub fn f32_to_decimal(n: f32) -> Decimal {
    Decimal::from_f32_retain(n).expect("Failed to convert f32 to decimal")
}

pub fn f64_to_decimal(n: f64) -> Decimal {
    Decimal::from_f64_retain(n).expect("Failed to convert f64 to decimal")
}

pub fn i32_to_bool(n: i32) -> bool {
    match n {
        x if x < 0 => {
            tracing::error!("Error when converting i32 to bool, {n} is smaller than 0");
            false
        }
        0 => false,
        1 => true,
        x if x > 1 => {
            tracing::error!("Error when converting i32 to bool, {n} is bigger than 1");
            true
        }
        _ => unreachable!(),
    }
}
