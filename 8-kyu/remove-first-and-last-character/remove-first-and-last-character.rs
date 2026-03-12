pub fn remove_char(s: &str) -> String {
​
    match s.len()
    {
        2 => return "".to_string(),
        _ => return s[1..s.len()-1].to_string(),
    }
}