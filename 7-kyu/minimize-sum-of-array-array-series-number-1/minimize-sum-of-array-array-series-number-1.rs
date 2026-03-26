fn min_sum(xs: &[u64]) -> u64 
{
    // My solution (Time: O(n log n), Space: O(n)): 1787 ms
    let mut x = xs.to_vec();
    x.sort(); // O(n log n)
    
    let n = x.len();
    
    (0..n / 2)
        .map(|i| x[i] * x[n - 1 - i]) // Pair smallest with largest
        .sum()
    
    // The Most Efficient Approach (Time: O(n), Space: O(n)) : 1731 ms
//     let mut x = xs.to_vec();
//     // 1. Sort in-place and "unstably" (doesn't preserve order of equal elements)
//     // This is significantly faster than a standard sort.
//     x.sort_unstable(); // O(n)
    
//     let n = x.len();
    
//     let mut total = 0;
​
//     // 2. Use a simple loop or twin pointers to avoid range overhead
//     for i in 0..n / 2 {
//         total += x[i] * x[n - 1 - i];
//     }
​
//     total
 
}