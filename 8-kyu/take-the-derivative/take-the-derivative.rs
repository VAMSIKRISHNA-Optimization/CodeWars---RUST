fn derive(coefficient: u32, exponent: u32) -> String 
{
    /* My Solution (Time: O(d), Space: O(d)): 1231 ms */ 
    format!("{}x^{}", coefficient*exponent, exponent-1)
    
    /* Better Solution (Time: O(d), Space: O(d)): 1234 ms */ 
//     let mut s = String::with_capacity(12); // Pre-allocate
//     s.push_str(&(coefficient * exponent).to_string());
//     s.push_str("x^");
//     s.push_str(&(exponent - 1).to_string());
//     s
​
}
​