fn solution(s: &str) -> String 
{
    if s.is_empty()
    {
        return "".to_string();
    }
    else
    {
        if s.chars().all(|c| c.is_lowercase())
        {
            return s.to_string();
        }
        else
        {
        
            let mut result       = Vec::new();
            let mut current_word = String::new();
            
            for c in s.chars() 
            {
                if c.is_uppercase() && !current_word.is_empty() 
                {
                    // Found an uppercase letter, and we have a word accumulated,
                    // so push the current word and start a new one.
                    result.push(current_word);
                    current_word = String::new();
                }
                current_word.push(c);
            }
            
            // Push the last accumulated word if it's not empty
            if !current_word.is_empty() 
            {
                result.push(current_word);
            }   
​
            return result.join(" ")
        }
    }
}