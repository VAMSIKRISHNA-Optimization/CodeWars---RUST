fn tidy_number(n: u64) -> bool
{
    /* My Solution (Time Complexity: O(D), Space Complexity: O(D)) : 1462 ms */
    n
    .to_string()
    .as_bytes()
    .windows(2)
    .all(|w| w[1]>=w[0])
    
    /* The Most Efficient Solution (Time Complexity: O(D), Space Complexity: O(1)) : 1416 ms */
//     let mut last_digit = 10; // Higher than any possible digit (0-9)
//     let mut n = n;
//     while n > 0 {
//         let current_digit = n % 10;
//         if current_digit > last_digit {
//             return false;
//         }
//         last_digit = current_digit;
//         n /= 10;
//     }
//     true
}