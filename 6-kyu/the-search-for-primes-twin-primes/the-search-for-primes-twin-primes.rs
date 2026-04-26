fn twin_prime(n:i32) -> u32 
{
    /* My Solution (Time: O(N * sqrt(N)), Space: O(1)): 1493 ms */
    if n <= 2 { return 0; }
    
    let mut twin_prime_count: u32 = 0;
    
    let mut num: u64 = 2;
    
    while num < n as u64
    {
        if is_prime(num) && is_prime(num+2)
        {
            twin_prime_count += 1;
        }
        
        if num%2 == 0 { num += 1; }
        else { num+=2; }
        
    }
    
    twin_prime_count
    
//     /* The Most Efficient Solution (Time: O(N log logN), Space: O(N)): 1496 ms */
//     if n <= 2 {
//         return 0;
//     }
    
//     // We treat 'n' as the cutoff for the FIRST number in the pair.
//     // So we need to check up to n + 2 to see if its partner is also prime!
//     let limit = (n as usize) + 2;
//     let mut twin_prime_count: u32 = 0;
​
//     // 1. Initialize Sieve of Eratosthenes up to n + 2
//     let mut is_prime = vec![true; limit];
//     if limit > 0 { is_prime[0] = false; }
//     if limit > 1 { is_prime[1] = false; }
​
//     let sqrt_limit = (limit as f64).sqrt() as usize;
//     for i in 2..=sqrt_limit {
//         if is_prime[i] {
//             let mut j = i * i;
//             while j < limit {
//                 is_prime[j] = false;
//                 j += i;
//             }
//         }
//     }
​