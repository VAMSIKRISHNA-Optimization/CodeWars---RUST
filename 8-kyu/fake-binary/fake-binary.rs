fn fake_bin(s: &str) -> String {
    s.chars().map(|c| if c.to_digit(10) < Some(5) { '0' } else { '1' }).collect()
}