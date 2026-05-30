fn switcher(numbers: Vec<&str>) -> String 
{
    /* My Solution : (Time Complexity: O(N), Space Complexity: O(N)) #Time: 1524 ms */  
    numbers.iter().map(|&s| mapper(s.parse::<u8>().unwrap()).unwrap()).collect::<String>()
    
//     /* The Most Efficient Solution : (Time Complexity: O(N), Space Complexity: O(N)) #Time: 1990 ms */
//     // Pre-allocate the exact string capacity to avoid costly dynamic heap reallocations
//     let mut result = String::with_capacity(numbers.len());
​
//     // Pre-computed branchless lookup table for inputs 0 to 29 
//     const LOOKUP: [char; 30] = 
//     [
//         '\0', 'z', 'y', 'x', 'w', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n', 'm', 
//         'l', 'k', 'j', 'i', 'h', 'g', 'f', 'e', 'd', 'c', 'b', 'a', '!', '?', ' '
//     ];
​
//     // FIX: Changed `&s` to `s` so that the type of `s` is safely `&str`
//     for s in numbers 
//     {
//         if let Ok(num) = s.parse::<usize>() 
//         {
//             if num < 30 {
//                 result.push(LOOKUP[num]);
//             }
//         }
//     }
​
//     result
}
​
fn mapper(num: u8) -> Option<char> 
{
    match num 
    {
        1..=26 => 
        {
            let offset = 26 - num;
            Some((b'a' + offset) as char)
        }
        27 => Some('!'),
        28 => Some('?'),
        29 => Some(' '),
        _ => None,
    }
}
​