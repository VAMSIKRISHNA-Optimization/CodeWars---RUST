fn solve(strings: &[String]) -> Vec<usize> 
{
    /* My Solution (Time Complexity: O(N*M), Space Complexity: O(N)) : 1523 ms */
    strings.
    iter()
    .map(|ss|
    {
        ss.
        chars().
        enumerate().
        fold(0, |acc,(i,c)| 
        {
            if c.to_ascii_lowercase() as u8 == (97u8 + i as u8) 
            {
                acc + 1
            }
            else
            { 
                acc
            }
        })
        
    })
    .collect::<Vec<usize>>()
    
    /* The Most Effective Solution (Time Complexity: O(N* min(M,26)) Space Complexity: O(N) : 1518 ms */
//     strings
//     .iter()
//     .map(|s| {
//         s.as_bytes() // Direct byte access, no UTF-8 decoding
//             .iter()
//             .enumerate()
//             .take(26) // Optimization: Stop after 'z'
//             .filter(|&(i, &byte)| (byte | 32) == 97 + i as u8)
//             .count()
//     })
//     .collect()
    
​
}