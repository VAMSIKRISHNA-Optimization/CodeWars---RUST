fn add_letters(letters: Vec<char>) -> char 
{
    /* My Solution (Time: O(N), Space: O(1)): 1383 ms */
    if letters.is_empty() { return 'z'; }
    
    let sum_chars = letters
                        .iter()
                        .map(|c| 
                        {
                            (*c as u8 - 'a' as u8) + 1
                        })
                        .sum::<u8>();
                        
    let mut next_ind = sum_chars % 26;
    if next_ind == 0 { next_ind = 26; }
    
    (next_ind + 'a' as u8 - 1) as char
    
//     /* The Most Efficient Solution (Time: O(N), Space: O(1)): 1338 ms */
//     // We sum as u32 to prevent overflow. 
//     // We subtract 1 initially to work in a 0-25 range for cleaner modulo logic.
//     let total_sum: u32 = letters
//         .iter()
//         .map(|&c| (c as u32 - 'a' as u32) + 1)
//         .sum();
​
//     // (total_sum - 1) % 26 finds the 0-indexed position.
//     // We use (sum + 25) % 26 to handle the 'empty vector' case and 'z' wrap-around.
//     // If letters is empty, total_sum is 0. (0 + 25) % 26 = 25. 25 + 'a' = 'z'.
//     let result_index = (total_sum + 25) % 26;
​
//     std::char::from_u32('a' as u32 + result_index).unwrap_or('z')
}
​