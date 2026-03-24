fn contamination(text: &str, character: &str) -> String 
{
    /* My Solution (Time Complexity: O(N), Space Complexity: O(M*N)) : 1584 ms */
    if text.is_empty() || character.is_empty() { return "".to_string(); }
    character.repeat(text.chars().count())
    
    /* The Most Effective Solution (Time Complexity: O(N+M), Space Complexity: O(M)) : 1512 ms */
//     if text.is_empty() || character.is_empty() {
//         return String::new();
//     }
​
//     // 1. Calculate how many times to repeat
//     let count = text.chars().count();
    
//     // 2. Pre-allocate the exact bytes needed to avoid resizing
//     let mut result = String::with_capacity(count * character.len());
    
//     // 3. Push the replacement string 'count' times
//     for _ in 0..count {
//         result.push_str(character);
//     }
    
//     result
}