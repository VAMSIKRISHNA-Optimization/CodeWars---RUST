​
​
fn is_prime(n: u32) -> bool 
{
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
​
    // Optimized Trial Division (6k ± 1) up to sqrt(n)
    let mut i = 5;
    while i * i <= n 
    {
        if n % i == 0 || n % (i + 2) == 0 
        {
            return false;
        }
        i += 6;
    }
    true
}
​
fn count_digits(num: u32) -> u32 
{
    let mut n = num;
    if n == 0 { return 1; }
    let mut count = 0;
    while n > 0 
    {
        n /= 10;
        count += 1;
    }
    count
}