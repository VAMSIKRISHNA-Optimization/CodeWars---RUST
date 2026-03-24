fn sort_my_string(s: &str) -> String 
{
    /* My Solution (Time Complexity: O(N), Space Complexity: O(N)) : 1707 ms (Mine is faster! Yay!) */
    
    // 1. Collect even-indexed characters
    let even: String = s.chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| c)
        .collect();
​
    // 2. Collect odd-indexed characters
    let odd: String = s.chars()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, c)| c)
        .collect();
​
    format!("{} {}", even, odd)
    
    /* The Most Effective Solution (Time Complexity: O(N), Space Complexity: O(N)) : 1754 ms */ 
//     let mut even = String::with_capacity(s.len() / 2 + 1);
//     let mut odd = String::with_capacity(s.len() / 2);
​
//     for (i, c) in s.chars().enumerate() {
//         if i % 2 == 0 {
//             even.push(c);
//         } else {
//             odd.push(c);
//         }
//     }
​
//     format!("{} {}", even, odd)
}