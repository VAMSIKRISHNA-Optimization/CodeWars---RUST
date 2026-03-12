/// Adds two extremely large number strings and returns the result as a String.
pub fn sum_strings(s1: &str, s2: &str) -> String {
    if s1.is_empty() &&  s2.is_empty()
    {
        return "0".to_string();
    }
    
    let mut num1: Vec<u8> = s1.chars().rev().filter_map(|c| c.to_digit(10).map(|d| d as u8)).collect();
    let mut num2: Vec<u8> = s2.chars().rev().filter_map(|c| c.to_digit(10).map(|d| d as u8)).collect();
    let mut result: Vec<u8> = Vec::new();
    let mut carry = 0;
​
    // Pad the shorter vector with zeros to make iteration easier
    if num1.len() < num2.len() { num1.resize(num2.len(), 0); }
    if num2.len() < num1.len() { num2.resize(num1.len(), 0); }
​
    for i in 0..num1.len() {
        let sum = num1[i] + num2[i] + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }
​
    if carry > 0 {
        result.push(carry);
    }
    
        while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }
​
​
    // Convert the result Vec<u8> back into a String in reverse order
    result.iter().rev().map(|&d| (d + b'0') as char).collect()
}
​