fn find_deleted_number(list: &[u16], mixed_list: &[u16]) -> Option<u16> 
{
    
    /* My Solution (Time Complexity: O(N*M), Space Complexity: O(1)) : 1642 ms */
    list
    .iter()
    .filter(|&n| !mixed_list.contains(n))
    .copied()
    .next()
    
    /* The Most Efficient Solution (Time Complexity: O(N), Space Complexity: O(1)) : 1456 ms */
//     // 1. Calculate sums using u64 to prevent overflow
//     let sum_original: u64 = list.iter().map(|&x| x as u64).sum();
//     let sum_mixed: u64 = mixed_list.iter().map(|&x| x as u64).sum();
​
//     // 2. The difference is the missing number
//     let diff = sum_original - sum_mixed;
​
//     if diff == 0 {
//         None
//     } else {
//         Some(diff as u16)
//     }
}