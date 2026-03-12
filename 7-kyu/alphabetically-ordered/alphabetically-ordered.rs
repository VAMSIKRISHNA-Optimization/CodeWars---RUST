fn alphabetic(s: &str) -> bool 
{
    if s.is_empty() 
    {
        return true;
    }
    else 
    {
        if s.len() == 1
        {
            return true;
        }
        else 
        {
            let mut all_ascii_values =vec![0; s.len()];
        
            for (ind,byte) in s.bytes().enumerate()
            {
                all_ascii_values[ind as usize] = byte;
            }
            
            let mut curr_ind = 0;
            while curr_ind < s.len()-1
            {
                if all_ascii_values[curr_ind+1] < all_ascii_values[curr_ind]
                {
                    return false;
                }
​
                curr_ind+= 1;
            }
​
            return true;
        }
​
    }
}