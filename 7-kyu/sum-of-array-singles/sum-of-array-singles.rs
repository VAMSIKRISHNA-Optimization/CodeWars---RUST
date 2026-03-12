fn repeats(arr: &Vec<i32>) -> i32 {
    let mut narr = arr.clone();
    narr.sort();
    let mut non_repeating = Vec::new();
​
    if !narr.is_empty() 
    {
    
        // Handle the first element
        if narr.len() == 1 || narr[0] != narr[1] 
        {
            non_repeating.push(narr[0]);
        }
​
        // Iterate through the middle elements
        for i in 1..(narr.len() - 1) 
        {
            if narr[i] != narr[i - 1] && narr[i] != narr[i + 1] 
            {
                non_repeating.push(narr[i]);
            }
        }
​
        // Handle the last element
        if narr.len() > 1 && narr[narr.len() - 1] != narr[narr.len() - 2] 
        {
            non_repeating.push(narr[narr.len() - 1]);
        }
        
    }
​
     non_repeating.iter().sum::<i32>() as i32
}