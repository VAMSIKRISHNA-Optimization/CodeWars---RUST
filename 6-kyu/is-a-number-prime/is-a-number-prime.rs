fn is_prime(n: i64) -> bool 
{
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
​
    let mut i = 5;
    // Only check up to the square root of n
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6; // Skip multiples of 2 and 3
    }
    true
}
​