    // Minutes 
    if all_counter[3] == 1 
    {
        duration.push_str(&format!("{} minute", all_counter[3]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    else if all_counter[3] > 1 
    {
        duration.push_str(&format!("{} minutes", all_counter[3]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    
    
    // Seconds 
    if all_counter[4] == 1 
    {
        duration.push_str(&format!("{} second", all_counter[4]));
    }
    else if all_counter[4] > 1 
    {
        duration.push_str(&format!("{} seconds", all_counter[4]));
    }
    
    //println!("{:?}", all_counter);
    
    duration
    
}