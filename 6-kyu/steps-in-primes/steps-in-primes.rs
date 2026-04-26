//     }
​
//     // 2. Scan the window linearly in O(K) time
//     for i in m..=(n - g) {
//         if is_prime[i] && is_prime[i + g] {
//             return Some((i as u64, (i + g) as u64));
//         }
//     }
​
//     None
}
​
fn is_prime(n: u64) -> bool 
{
​
    if n <= 1 { return false; }
    if n <= 3 { return true; } 
    if n % 2 == 0 || n % 3 == 0 { return false; }
​
    let mut i = 5;
    while i * i <= n 
    {
​
        if n % i == 0 || n % (i + 2) == 0 
        {
            return false;
        }
        i += 6; 
    }
​
    true
}
​