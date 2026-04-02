use std::collections::HashMap;
​
fn my_languages(results: HashMap<&str, i32>) -> Vec<&str> 
{
    /* My Solution (Time Complexity: O(N log N), Space Complexity: O(K)) : 1732 ms */
    let mut filtered_results: Vec<(&&str, &i32)> = results
                                                    .iter()
                                                    .filter(|&(_, &v)| v >= 60)
                                                    .collect();
​
    // Sort by value in descending order (highest score first)
    filtered_results.sort_by(|a, b| b.1.cmp(a.1));
​
    // Return the vec
    filtered_results
        .into_iter()
        .map(|(&k, _)| k)
        .collect()
    
//     /* The Most Efficient Solution (Time Complexity: O(N log N), Space Complexity: O(K)) : 1740 ms */
//     // 1. Filter and collect directly into the final Vec
//     let mut filtered: Vec<(&str, i32)> = results
//         .into_iter()
//         .filter(|&(_, v)| v >= 60)
//         .collect();
​
//     // 2. Sort unstable (faster as it doesn't preserve order of equal elements)
//     // We sort by value (v) in descending order
//     filtered.sort_unstable_by(|a, b| b.1.cmp(&a.1));
​
//     // 3. Transform the Vec in-place to just the strings
//     filtered.into_iter().map(|(k, _)| k).collect()
​
}