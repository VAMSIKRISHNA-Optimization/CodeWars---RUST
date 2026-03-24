fn format_money(amount: f64) -> String 
{
    /* My Solution (Time Complexity: O(DIGITS), Space Complexity: O(DIGITS)) : 1466 ms */
    format!("${:.02}", amount)
    
    /* The Most Effective Solution (Time Complexity: O(DIGITS), Space Complexity: O(DIGITS)) : 1431 ms */
    // Rounding to 2 decimal places manually
//     let mut s = String::with_capacity(12); // Pre-allocate to avoid resizing
//     s.push('$');
//     let rounded = format!("{:.2}", amount); // 'dtoa' crate would be even faster here
//     s.push_str(&rounded);
//     s
}