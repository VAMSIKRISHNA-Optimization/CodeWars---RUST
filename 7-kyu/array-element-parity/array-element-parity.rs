use std::collections::HashSet;
fn solve(arr: &Vec<i32>) -> i32 
{
    /* My Solution : (Time Complexity: O(N^2), Space Complexity: O(N^2)) #Time: 2282 ms */
    for val in arr.iter()
    {
        if arr.iter().find(|&&x| x == -1*val).is_none()
        {
            return *val;
        }
    }
    
    0 as i32
    
//     /* The Most Efficient Solution : (Time Complexity: O(N), Space Complexity: O(1)) #Time: 2217 ms */
//     // Step 1: Dedup into a HashSet for O(1) lookups
//     let elements: HashSet<&i32> = arr.iter().collect();
​
//     // Step 2: Find the element whose negative counterpart doesn't exist
//     for &val in arr {
//         if !elements.contains(&(-1 * val)) {
//             return val;
//         }
//     }
​
//     0
}
​