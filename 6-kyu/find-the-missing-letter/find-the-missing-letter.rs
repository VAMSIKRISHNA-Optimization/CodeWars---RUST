fn find_missing_letter(chars: &[char]) -> char 
{
    for ind in 0..(chars.len()-1)as usize
    {
        if (chars[ind+1] as u32) != (chars[ind] as u32) + 1 
        {
             return char::from_u32((chars[ind] as u32 + 1)).expect("FAIL");;
        }
    }
    
    '\0'
}