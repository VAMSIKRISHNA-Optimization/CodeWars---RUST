use std::collections::HashSet;
fn is_pangram(s: &str) -> bool 
{
    let char_map: HashSet<char> = s.chars().flat_map(|c| c.to_uppercase()).collect();
    ('A'..='Z').all(|c| char_map.contains(&c))
}