fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    (holiday_cost as f64 / (price as f64 * (discount as f64 / 100.0))) as i32
}