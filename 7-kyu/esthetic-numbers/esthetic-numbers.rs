fn esthetic(num: u32) -> Vec<u8> 
{
    let mut bases_matched: Vec<u8> = Vec::new();
    
    for base in 2..=10
    {
            let base_rep = match base 
            {
                2  => format!("{:b}", num), // Fast built-in binary
                8  => format!("{:o}", num), // Fast built-in octal
                10 => num.to_string(),      // Fast built-in decimal
                _  => to_base_string(num, base), // Custom for 3, 4, 5, 6, 7, 9
            };
        
        let chars_base: Vec<char> = base_rep.chars().collect();
        
        if chars_base.len() < 2 
        {
            bases_matched.push(base as u8);
            continue;
        }
        
        let chk = chars_base.windows(2).all(|w| 
            {
                let d1 = w[0].to_digit(10).unwrap() as i32;
                let d2 = w[1].to_digit(10).unwrap() as i32;
                (d1 - d2).abs() == 1
            });
​
        if chk { bases_matched.push(base as u8); }
        
    }
   
    bases_matched
}
​
// The efficient way to build a string for any base
fn to_base_string(mut n: u32, base: u32) -> String 
{
    if n == 0 { return "0".to_string(); }
    let mut result = String::with_capacity(32); 
    while n > 0 {
        // Use char::from_digit to turn 0-9 into '0'-'9'
        let digit = std::char::from_digit(n % base, base).unwrap();
        result.push(digit);
        n /= base;
    }
    // Result is built backwards, so reverse it
    result.chars().rev().collect()
}