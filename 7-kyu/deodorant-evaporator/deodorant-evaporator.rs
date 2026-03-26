fn evaporator(content: f64, evap_per_day: i32, threshold: i32) -> i32 
{
    // My Solution (Time: O(log(threshold)/log(1-evap_per_day)), Space: O(1)) : 1280 ms
    let ths_ml = ((threshold as f64)/100.0) * content;
    let mut con_rem_ml = content;
    let mut days = 0;
    
    while con_rem_ml >= ths_ml
    {
        con_rem_ml = con_rem_ml -  (((evap_per_day as f64)/100.0) * con_rem_ml);
        days += 1;
    }
    days
    
    // The Most Effective Solution: Logarithmic Calculation (Time: O(1), Space: O(1)) : 1180 ms
//     let rate = 1.0 - (evap_per_day as f64 / 100.0);
//     let limit = threshold as f64 / 100.0;
    
//     // We solve: rate^days < limit
//     // Using logs: days * ln(rate) < ln(limit)
//     // Since ln(rate) is negative, the inequality flips:
//     // days > ln(limit) / ln(rate)
    
//     (limit.ln() / rate.ln()).ceil() as i32
}