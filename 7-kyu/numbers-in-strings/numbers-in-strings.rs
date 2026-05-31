fn solve(s: &str) -> u32 
{
    /* My Solution : (Time Complexity: O(N), Space Complexity: O(N)) #Time: 2523 ms */ 
    *(s
    .split(|c: char| c.is_ascii_alphabetic())
    .filter_map(|s| s.parse::<u32>().ok())
    .collect::<Vec<u32>>()
    .iter()
    .max()
    .unwrap_or(&0))
    
//     /* The Most Efficient Solution : (Time Complexity: O(N), Space Complexity: O(1)) #Time: 2423 ms */
//     s
//     .split(|c: char| c.is_ascii_alphabetic())
//     .filter_map(|sub| sub.parse::<u32>().ok())
//     .max()
//     .unwrap_or(0)
}