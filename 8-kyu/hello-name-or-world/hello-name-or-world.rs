fn hello(name: &str) -> String 
{
//     // My solution (Time: O(n), Space: O(n)) : 1566 ms 
    if name.is_empty()
    {
        format!("Hello, World!")
    }
    else
    {
        format!("Hello, {}{}!", name[..1].to_uppercase(), name[1..].to_lowercase())
    }
    
    // Faster and safer solution where format! is avoided (Time: O(n), Space: O(n)) : 1313 ms
//     if name.is_empty() {
//         return "Hello, World!".to_string();
//     }
​
//     // Pre-allocate space: "Hello, " (7) + name len + "!" (1)
//     let mut result = String::with_capacity(name.len() + 8);
//     result.push_str("Hello, ");
​
//     let mut chars = name.chars();
//     if let Some(first) = chars.next() {
//         // Uppercase the first char safely (some chars uppercase to multiple chars)
//         for c in first.to_uppercase() {
//             result.push(c);
//         }
//     }
​
//     // Lowercase the rest safely
//     for c in chars {
//         for low_c in c.to_lowercase() {
//             result.push(low_c);
//         }
//     }
​
//     result.push('!');
//     result
}