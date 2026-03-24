fn is_perfect_power(n: u64) -> Option<(u64, u32)> 
{
    
    let mut factors: Vec<u64> = Vec::new();
    let mut i: u64 = 1;
​
    // Loop up to the square root of the number
    while i * i <= n {
        if n % i == 0 {
            // 'i' is a factor
            factors.push(i);
​
            // If the factors are not the same (e.g., if number is a perfect square),
            // then number/i is a different factor and should also be added.
            if i * i != n {
                factors.push(n / i);
            }
        }
        i += 1;
    }
​
    // Sort the factors for a clean, ordered list
    factors.sort();
    
    
    for val in factors
    {
        for pow in 2..10
        {
            if ((val as u128).pow(pow as u32))as u128 > n as u128 
            {   
                break;   
            }
            else
            {
                 if ((val as u64).pow(pow as u32)) as u128 == n as u128 { return Some((val as u64, pow as u32)); }
            }
        }
    }
    return None;
}