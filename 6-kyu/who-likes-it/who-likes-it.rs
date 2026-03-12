fn likes(names: &[&str]) -> String {
    if names.is_empty()
    {
        return "no one likes this".to_string();
    }
    else
    {
        match names.len()
        {
            1 => 
            {
                let ans: String = format!("{} likes this", names[0]);
                return ans;
            }
            
            2 => 
            {
                let ans: String = format!("{} and {} like this", names[0], names[1]);
                return ans;
            }
            
            3 => 
            {
                let ans: String = format!("{}, {} and {} like this",  names[0], names[1],  names[2]);
                return ans;
            }
            _ => 
            {
                let other_count = names.len()-2;
                let ans: String = format!("{}, {} and {} others like this",  names[0], names[1], other_count);
                return ans;
            }
        }
    }
}