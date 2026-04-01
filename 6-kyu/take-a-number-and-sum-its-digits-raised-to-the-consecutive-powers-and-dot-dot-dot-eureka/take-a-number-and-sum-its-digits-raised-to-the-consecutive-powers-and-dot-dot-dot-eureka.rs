//             let mut len = 0;
//             let mut temp = n;
//             while temp > 0 {
//                 temp /= 10;
//                 len += 1;
//             }
​
//             // 2. Extract digits right-to-left
//             let mut temp = n;
//             let mut sum: u64 = 0;
//             for i in (1..=len).rev() {
//                 let digit = temp % 10;
//                 // Using checked_add to be safe against overflows
//                 sum = sum.saturating_add(digit.pow(i));
//                 temp /= 10;
//             }
//             sum == n
//         })
//         .collect()
}
​
// fn get_len(mut n: u64) -> u32 {
//     if n == 0 { return 1; }
//     let mut len = 0;
//     while n > 0 {
//         n /= 10;
//         len += 1;
//     }
//     len
// }