fn strong(n: u64) -> String 
{
    /* My Solution (Time: O(D), Space: O(D)): 1369 ms */
    if sum_fact(n) == n { return "STRONG!!!!".to_string(); }
    else { return "Not Strong !!".to_string(); }
    
//      /* The Most Efficient Solution (Time: O(D), Space: O(1)): 1413 ms */
//     // Pre-computed factorials for 0! through 9!
//     const FACT_TABLE: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    
//     let mut sum = 0;
//     let mut temp = n;
​
//     // Edge case for 0
//     if n == 0 { sum = FACT_TABLE[0]; }
​
//     while temp > 0 {
//         let digit = (temp % 10) as usize;
//         sum += FACT_TABLE[digit];
//         temp /= 10;
//     }
​
//     if sum == n { "STRONG!!!!".to_string() } 
//     else { "Not Strong !!".to_string() }
}
​
fn sum_fact(mut n: u64) -> u64
{
    get_digits(n).iter().map(|&v| factorial(v)).sum::<u64>()
}
​
fn factorial(n: u64) -> u64 
{
    (1..=n).product()
}
​
fn get_digits(mut n: u64) -> Vec<u64> 
{
    if n == 0 { return vec![0]; }
    let mut digits = Vec::with_capacity(20); // u64 has max 20 digits
    while n > 0 
    {
        digits.push((n % 10) as u64);
        n /= 10;
    }
    digits.reverse(); // Optional: only if you need them in original order
    digits
}