fn evens_and_odds(n: u64) -> String 
{
    /* My Solution : (Time Complexity: O(D), Space Complexity: O(D)) #Time: 1062 ms */  
    if n % 2 == 0
    {
        format!("{:b}", n)
    }
    else
    {
        format!("{:x}", n)
    }
    
//     /* The Most Efficient Solution : (Time Complexity: O(D), Space Complexity: O(D)) #Time: 1125 ms */
//     if n == 0 {
//         return String::from("0");
//     }
​
//     if n % 2 == 0 {
//         // Binary optimization: max 64 bits for u64
//         let mut result = String::with_capacity(64);
//         let mut temp = n;
//         let mut buffer = [0u8; 64];
//         let mut cursor = 64;
​
//         while temp > 0 {
//             cursor -= 1;
//             buffer[cursor] = b'0' + (temp & 1) as u8;
//             temp >>= 1; // Bitwise shift is faster than division
//         }
        
//         // Safe conversion since we only populated ASCII '0' and '1'
//         let s = std::str::from_utf8(&buffer[cursor..]).unwrap();
//         result.push_str(s);
//         result
//     } else {
//         // Hexadecimal optimization: max 16 digits for u64
//         let mut result = String::with_capacity(16);
//         let mut temp = n;
//         let mut buffer = [0u8; 16];
//         let mut cursor = 16;
//         const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
​
//         while temp > 0 {
//             cursor -= 1;
//             buffer[cursor] = HEX_DIGITS[(temp & 0xF) as usize];
//             temp >>= 4; // Bitwise shift by 4 for hex digits
//         }
​
//         let s = std::str::from_utf8(&buffer[cursor..]).unwrap();
//         result.push_str(s);
//         result
//     }
}