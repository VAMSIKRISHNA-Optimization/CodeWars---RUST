fn change(string: &str) -> String 
{
    let mut s = "0".repeat(26);
    string.chars().for_each(|c| 
    {
        if c.is_ascii_alphabetic()
        {
            let char_val = (c.to_ascii_lowercase() as u8 - b'a' + 1) as usize;
            s.replace_range(char_val-1..char_val, "1");
        }
    });
    s
}