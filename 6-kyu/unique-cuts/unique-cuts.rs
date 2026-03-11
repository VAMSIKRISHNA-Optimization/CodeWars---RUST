fn split_unique_substrings(s: &str) -> Vec<usize> 
{
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let mut result = Vec::new();
    let mut i = 0;
​
    while i < chars.len() {
        let mut boundary = i;
        let mut check_idx = i;
​
        // Keep expanding the boundary as long as characters 
        // inside the segment have occurrences further out.
        while check_idx <= boundary {
            let current_char = chars[check_idx].1;
            
            // Find the ABSOLUTE LAST occurrence of this character in the whole string
            if let Some(last_pos) = chars.iter().rposition(|&(_, c)| c == current_char) {
                if last_pos > boundary {
                    boundary = last_pos;
                }
            }
            check_idx += 1;
        }
​
        // Calculate segment length in characters
        let start_byte = chars[i].0;
        let (end_byte, end_char) = chars[boundary];
        let segment_end = end_byte + end_char.len_utf8();
        
        result.push(s[start_byte..segment_end].chars().count());
        
        // Move to the next character after the boundary
        i = boundary + 1;
    }
    result
}