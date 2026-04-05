use std::collections::HashMap;
​
fn cannons_ready(gunners: HashMap<&str, &str>) -> String 
{
    /* MY SOLUTION - Also, the most efficient! Yay!!! ( Time: O(N) , Space: O (1) ): 1561 ms*/ 
    if gunners.iter().all(|(_,&v)| v=="aye")
    {
        "Fire!".to_string()
    }
    else
    {
        "Shiver me timbers!".to_string()
    }
}