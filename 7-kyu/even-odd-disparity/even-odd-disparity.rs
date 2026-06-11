fn solve(v: &Vec<String>) -> i32 
{
    /* My Solution : (Time Complexity: O(N x M), Space Complexity: O(1)) #Time: 1733 ms */  
    let e_o_counts = v
                    .iter()
                    .fold((0 as i32,0 as i32), |mut acc,s|
                    {
                        if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
                        {
                            if s.parse::<i32>().unwrap() % 2 == 0 { acc.0 += 1; }
                            else { acc.1  +=1; }
                        }
                        acc
                    });
    e_o_counts.0 - e_o_counts.1
    
//     /* The Most Efficient Solution : (Time Complexity: O(N x M), Space Complexity: O(1)) #Time: 1301 ms */
//     let mut difference = 0;
​
//     for s in v {
//         // Single pass: tries to parse directly. Non-digits fail automatically.
//         if let Ok(num) = s.parse::<i32>() {
//             // Bitwise AND is faster than modulo. 
//             // If the last bit is 0, the number is even.
//             if num & 1 == 0 {
//                 difference += 1;
//             } else {
//                 difference -= 1;
//             }
//         }
//     }
​
//     difference
}