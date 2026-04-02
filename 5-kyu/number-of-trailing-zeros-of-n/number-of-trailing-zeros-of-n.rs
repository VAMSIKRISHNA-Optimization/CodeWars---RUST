fn zeros(mut n: u64) -> u64 
{
    /* My Solution (Time: O(log5 N), Space: O(1)): 1416 ms */
    if n == 0 { return 0; }
    
    let mut tz = 0;
    let mut pow_5 = 5;
    
    while n/pow_5 > 0
    {
        tz += n/pow_5;
        pow_5 *= 5;
    }
    
    tz
    
//     /* The Most Efficient Solution (Time: O(log5 N), Space: O(1)): 1219 ms */
//     let mut tz = 0;
    
//     // Instead of multiplying a denominator up, 
//     // divide n down to calculate n/5, n/25, etc.
//     while n >= 5 {
//         n /= 5;
//         tz += n;
//     }
    
//     tz
}