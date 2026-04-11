fn max_tri_sum(arr: &[i32]) -> i32 
{
    /* MY SOLUTION ( Time: O(N log N) , Space: O (N) ): 1826 ms*/ 
    let mut vec_arr = arr.to_vec();
    vec_arr.sort_unstable();
    vec_arr.dedup();
    vec_arr[vec_arr.len()-1] + vec_arr[vec_arr.len()-2] + vec_arr[vec_arr.len()-3]
    
//     /* THE MOST EFFICIENT SOLUTION ( Time: O(N) , Space: O(1) ): 1728 ms*/ 
//     let mut m1 = i32::MIN; // Largest
//     let mut m2 = i32::MIN; // Second largest
//     let mut m3 = i32::MIN; // Third largest
​
//     for &num in arr {
//         // Skip duplicates to ensure unique elements
//         if num == m1 || num == m2 || num == m3 {
//             continue;
//         }
​
//         if num > m1 {
//             m3 = m2;
//             m2 = m1;
//             m1 = num;
//         } else if num > m2 {
//             m3 = m2;
//             m2 = num;
//         } else if num > m3 {
//             m3 = num;
//         }
//     }
​
//     m1 + m2 + m3
}