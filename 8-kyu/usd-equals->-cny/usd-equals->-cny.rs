use std::fmt::Write;
fn usdcny(usd: u16) -> String 
{
    /* My Solution (Time Complexity: O(N*M), Space Complexity: O(N*M)) : 1230 ms */
    // 1. Pre-allocate String for speed
    let mut ans = String::with_capacity(25); // Pre-allocates 25 bytes on the heap
    
    // 2. Calculate the conversion
    let conv: f32 = usd as f32 * 6.75_f32;
    
    // 3. Push the integer part to the pre-allocated string
    let integer_part = conv.trunc() as u32;
    ans.push_str(&integer_part.to_string());
    ans.push('.');
​
    // 4. Push the fractional part with only 2 decimals
    let fractional_part = ((conv.abs() - conv.abs().trunc()) * 100.0).round() as u16;
​
    // 5. Add pre-padding zero if necessary
    if fractional_part < 10 
    {
        ans.push('0');
    }
    ans.push_str(&fractional_part.to_string());
    
    // 6. Conditioning the string 
    ans.push_str(" Chinese Yuan");
    ans
    
    /* The Most Effective Solution (Time Complexity: O(N+M), Space Complexity: O(Output Size)) : 1425 ms */
//     // 1. Pre-allocate exactly what's needed
//     let mut s = String::with_capacity(30); 
    
//     // 2. Use Fixed-Point math (Integers are faster than Floats)
//     // 6.75 * 100 = 675. We work in "cents" to avoid float precision issues.
//     let total_fen = usd as u32 * 675;
//     let yuan = total_fen / 100;
//     let fen = total_fen % 100;
​
//     // 3. Write directly into the buffer (No temporary strings)
//     let _ = write!(s, "{}.{:02} Chinese Yuan", yuan, fen);
    
//     s
}