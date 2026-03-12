pub fn get_keyword(ciphertext: &str, key_len: usize) -> String {
    (0..key_len)
        .map(|pos| {
            let chars: Vec<char> = ciphertext.chars().skip(pos).step_by(key_len).collect();
            (0..26)
                .max_by(|&a, &b| {
                    score(&chars, a).partial_cmp(&score(&chars, b)).unwrap()
                })
                .map(|shift| (b'A' + shift as u8) as char)
                .unwrap()
        })
        .collect()
}
​
fn score(chars: &[char], shift: u8) -> f64 {
    let freq = [0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015,
                0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749,
                0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758,
                0.00978, 0.02360, 0.00150, 0.01974, 0.00074];
    
    let mut counts = [0; 26];
    for &c in chars {
        counts[((c as u8 - b'A' + 26 - shift) % 26) as usize] += 1;
    }
    
    let total = chars.len() as f64;
    counts.iter().enumerate()
        .map(|(i, &count)| {
            let observed = count as f64 / total;
            -(observed - freq[i]).powi(2) / freq[i]
        })
        .sum()
}