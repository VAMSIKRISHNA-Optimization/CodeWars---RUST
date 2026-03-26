fn capitalize(s: &str) -> Vec<String> 
{
    let mut ans: Vec<String> = vec!["".to_string(),"".to_string()];
    
    s.chars()
        .enumerate()
        .for_each(|(ind,c)| 
        {
            if ind%2 == 0 
            { 
                ans[0].extend(c.to_uppercase());
                ans[1].extend(c.to_lowercase()); 
            } 
            else 
            { 
                ans[0].extend(c.to_lowercase());
                ans[1].extend(c.to_uppercase()); 
            }
​
        });
    
    ans
}