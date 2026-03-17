fn hex_to_dec(hex_string: &str) -> u32 
{
    // My Solution (Time: O(n), Space: O(1)) : 1351ms
    hex_string
    .chars()
    .rev()
    .enumerate()
    .map(|(i,h)| ind_hex_to_dec(h)*16_u32.pow(i as u32))
    .sum::<u32>()
     
    // The Most Efficient Solution (Time: O(n), Faster, Space: O(1)) : 1346ms
//     hex_string.chars().fold(0, |acc, c| 
//     {
//         (acc << 4) + ind_hex_to_dec(c)
//     })
    
}
​
fn ind_hex_to_dec (hex: char) -> u32 
{
    match hex
    {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        '0' => 0,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => panic!("Enter a valid HEX number!"),
    }
}