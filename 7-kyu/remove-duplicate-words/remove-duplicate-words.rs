use std::collections::HashSet;
​
fn remove_duplicate_words(s: &str) -> String 
{
    let mut seen = HashSet::new();
    s.split_whitespace()
        .filter(|&word| seen.insert(word)) // Returns true only if word was NOT in set
        .collect::<Vec<_>>()
        .join(" ")
}