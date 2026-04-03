fn html_special_chars(html: &str) -> String 
{
    /* MY SOLUTION ( Time: O(N * K) , Space: O (N) ): 1487 ms*/ 
    html
    .replace('&', "&amp;")
    .replace('"', "&quot;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    
//     /* THE MOST EFFICIENT SOLUTION ( Time: O(N) , Space: O(N) ): 1475 ms*/ 
//     // Pre-allocate space to reduce reallocations (heuristic: 10% extra)
//     let mut result = String::with_capacity(html.len() + (html.len() / 10));
    
//     for c in html.chars() {
//         match c {
//             '&' => result.push_str("&amp;"),
//             '"' => result.push_str("&quot;"),
//             '<' => result.push_str("&lt;"),
//             '>' => result.push_str("&gt;"),
//             _ => result.push(c),
//         }
//     }
//     result
}