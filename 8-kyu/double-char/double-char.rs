fn double_char(s: &str) -> String {
    s.chars().map(|c| format!("{}{}",c,c)).collect::<Vec<_>>().join("")
}