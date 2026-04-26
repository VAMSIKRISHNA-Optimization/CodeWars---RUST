use std::iter::repeat;
fn rot(s: &str) -> String 
{
    
    /* My Solution (Time: O(N), Space: O(N)) */
    s
    .chars()
    .rev()
    .map(|c| c.to_string())
    .collect::<String>()
    
//     /* The Most Efficient Solution (Time: O(N), Space: O(N)) */
//     s.chars().rev().collect()
}
​
​
fn selfie_and_rot(s: &str) -> String 
{
    /* My Solution (Time: O(N), Space: O(N)) */
    let s_dot = s
                .lines()
                .map(|ss| format!("{}{}", ss, repeat('.').take(ss.len()).collect::<String>()) )
                .collect::<Vec<String>>()
                .join("\n");
    
    let s_rev_dot = s_dot
                    .chars()
                    .rev()
                    .collect::<String>();
    
    format!("{}\n{}", s_dot, s_rev_dot)
    
//     /* The Most Efficient Solution (Time: O(N), Space: O(N)) */
//     // 1. Calculate the exact final size to perform exactly 1 allocation
//     // Original length + dots (same as orig) + newline for every line
//     let original_len = s.len();
    
//     // Total capacity = Top half + Middle newline + Bottom half
//     // Top half size = original_len (chars) + original_len (dots) + s.lines().count() (newlines)
//     let top_len = original_len * 2 + s.lines().count();
//     let total_capacity = top_len * 2; 
    
//     let mut result = String::with_capacity(total_capacity);
​
//     // 2. Build the top half directly into the buffer
//     for line in s.lines() {
//         result.push_str(line);
//         result.extend(repeat('.').take(line.len()));
//         result.push('\n');
//     }
    
//     // Remove the trailing newline from the top half
//     result.pop(); 
​
//     // 3. Mirror the top half directly into the bottom half
//     let bottom_half: String = result.chars().rev().collect();
    
//     result.push('\n');
//     result.push_str(&bottom_half);
​
//     result
}
​
​
// first parameter: dots have to be replaced by function of one variable
fn oper(f: fn(&str) -> String, s: &str) -> String 
{
    f(s)
}