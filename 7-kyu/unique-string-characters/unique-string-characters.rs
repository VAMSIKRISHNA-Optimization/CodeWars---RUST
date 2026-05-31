use std::collections::HashSet;
fn solve(a: &str, b: &str) -> String 
{
    /* My Solution : (Time Complexity: O(N+M), Space Complexity: O(Ua + Ub)) #Time: 1698 ms */  
    // Also, THE MOST OPTIMAL SOLUTION (YaY!)
    let s1_hs: HashSet<char> = a.chars().collect();
    let s2_hs: HashSet<char> = b.chars().collect();
​
    let mut result = String::new();
​
​
    for c in a.chars() 
    {
        if !s2_hs.contains(&c) 
        {
            result.push(c);
        }
    }
​
    for c in b.chars() 
    {
        if !s1_hs.contains(&c) 
        {
            result.push(c);
        }
    }
​
    result
    
}
​