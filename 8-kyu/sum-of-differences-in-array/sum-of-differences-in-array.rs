fn sum_of_differences(arr: &[i8]) -> Option<i8> 
{
    /* My Solution (Time Complexity: O(N log N), Space Complexity: O(N)) : 1731 ms */
    if arr.is_empty() || arr.len() == 1 { return None; }
    
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(|a, b| b.cmp(a)); // Sort descending
    Some(sorted_arr.windows(2).map(|w| w[0]-w[1]).sum::<i8>())
    
    /* The Most Effective Solution (Time Complexity: O(N), Space Complexity: O(1)) : 1482 ms */
//     if arr.len() < 2 { return None; }
​
//     // One pass to find max and min
//     let mut min = i8::MAX;
//     let mut max = i8::MIN;
​
//     for &val in arr {
//         if val < min { min = val; }
//         if val > max { max = val; }
//     }
​
//     // Return the difference
//     Some(max - min)
    
}