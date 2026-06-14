fn score(n: u32) -> u32 
{
    // Timedout- My Solution
//     (1..=n).fold(0, |acc, v| acc | v)
    
    // Catch 0 early so leading_zeros never becomes 32
    if n == 0 { return 0; }
    
    // Count how many leading zeros are at the front of the number
    let leading_zeros = n.leading_zeros();
    
    // Shift the maximum u32 value to perfectly wrap and cover all active bits
    u32::MAX >> leading_zeros
}
​