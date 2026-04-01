fn add_length(s: &str) -> Vec<String> {
    s.split_whitespace().map(|wrd| format!("{} {}", wrd, wrd.len())).collect::<Vec<String>>()
}