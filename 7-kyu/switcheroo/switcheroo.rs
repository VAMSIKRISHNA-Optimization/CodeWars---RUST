fn switcheroo(s: &str) -> String 
{
    // My Solution (Time: O(n), Space: O(n)): 1401 ms
    s
    .chars()
    .map(|c|
    {
        if c == 'a' { 'b'.to_string() } 
        else if c == 'b' { 'a'.to_string() }
        else { c.to_string() }
        
    })
    .collect::<String>()
    
    // Most Efficient Approach (Time: O(n), Space: O(n)): 1305 ms
//     s.chars()
//     .map(|c| match c {
//         'a' => 'b',
//         'b' => 'a',
//         _ => c,
//     })
//     .collect()
}