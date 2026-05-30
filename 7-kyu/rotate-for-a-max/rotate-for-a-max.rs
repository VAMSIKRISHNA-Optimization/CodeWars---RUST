fn max_rot(n: u64) -> u64 
{
    /* My Solution : (Time Complexity: O(D^2), Space Complexity: O(D^2)) #Time: 1372 ms */  
    let n_str = n.to_string();
    let len   =  n_str.len();
    
    let mut nums = vec![n];
    let mut n2: String = rotate_left(&n_str,1);
​
    
    nums.push(n2.parse::<u64>().unwrap_or(0));
    
    for i in 0..len.saturating_sub(2)
    {
        let rem = rotate_left(&n2[i+1..], 1);
        let n_s = format!("{}{}",&n2[..i+1],rem);
        
        nums.push(n_s.clone().parse::<u64>().unwrap_or(0));
        n2 = n_s;
    }
    
    *nums.iter().max().unwrap()
    
//     /* The Most Efficient Solution : (Time Complexity: O(D^2), Space Complexity: O(1)) #Time: 1228 ms */
//     // A 64-bit integer has at most 20 digits.
//     // We can use a fixed-size array on the stack to avoid heap allocations.
//     let mut digits = [0u8; 20];
//     let mut len = 0;
    
//     // Extract digits in reverse order arithmetically
//     let mut temp = n;
//     if temp == 0 {
//         len = 1;
//     } else {
//         while temp > 0 {
//             digits[len] = (temp % 10) as u8;
//             len += 1;
//             temp /= 10;
//         }
//         // Reverse the array slice to get the correct left-to-right digit order
//         digits[..len].reverse();
//     }
​
//     let mut max_val = n;