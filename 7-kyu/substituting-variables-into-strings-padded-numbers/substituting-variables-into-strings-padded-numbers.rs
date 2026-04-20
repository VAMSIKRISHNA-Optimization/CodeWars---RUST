fn solution(n: u32) -> String 
{
    /* My Solution (Time: O(D), Space: O(D)): 1340 ms */
    format!("Value is {:05}", n)
    
//     /* The Most Efficient Solution (Time: O(D), Space: O(D)): 1326 ms */
//     // "Value is " (9) + "00000" (5) = 14 bytes
//     let mut s = String::with_capacity(14);
//     s.push_str("Value is ");
    
//     let val_str = n.to_string();
//     // Add leading zeros if the number has fewer than 5 digits
//     for _ in 0..5_isize.saturating_sub(val_str.len() as isize) {
//         s.push('0');
//     }
    
//     s.push_str(&val_str);
//     s
}