fn find_digit(num: i32, nth: i32) -> i32 
{
    /* My Solution (Time Complexity: O(D), Space Complexity: O(D)) : 1499 ms */
    if nth <= 0 { return -1; }
​
    num
    .abs()
    .to_string()
    .chars()
    .rev()
    .nth((nth-1) as usize)
    .map(|c| c.to_digit(10).unwrap() as i32)
    .unwrap_or(0)
    
    /* The Most Efficient Solution (Time Complexity: O(D), Space Complexity: O(1)) : 1191 ms */
//     if nth <= 0 { return -1; }
​
//     let mut n = num.abs() as u32;
    
//     // We want the nth digit from the right.
//     // Loop (nth - 1) times to "shift" the number to the right.
//     for _ in 1..nth {
//         if n == 0 { return 0; } // The number is shorter than nth
//         n /= 10;
//     }
​
//     (n % 10) as i32
}