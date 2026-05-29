fn calc(s: &str) -> u32
{
    /* My Solution : (Time Complexity: O(N), Space Complexity: O(N)) #Time: 1388 ms */   
    let t1 = s.chars().map(|c| (c as u8).to_string()).collect::<Vec<String>>().join("");
    let t2 = t1.replace('7', "1");
    
    t1.bytes().map(|b| (b - b'0') as u32).sum::<u32>() - t2.bytes().map(|b| (b - b'0') as u32).sum::<u32>()
    
    
//     /* The Most Efficient Solution : (Time Complexity: O(N), Space Complexity: O(1)) #Time: 1808 ms */
//     let mut count_of_sevens = 0;
​
//     for b in s.bytes() {
//         let mut code = b;
//         // Extract each digit arithmetically from the ASCII code (e.g., 65 -> 5, then 6)
//         while code > 0 {
//             if code % 10 == 7 {
//                 count_of_sevens += 1;
//             }
//             code /= 10;
//         }
//     }
​
//     count_of_sevens * 6
}