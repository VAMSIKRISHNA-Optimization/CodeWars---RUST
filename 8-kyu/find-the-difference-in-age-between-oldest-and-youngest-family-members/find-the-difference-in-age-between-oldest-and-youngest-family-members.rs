fn difference_in_ages(ages: &[u8]) -> (u8, u8, u8) 
{
    /* My Solution (Time Complexity: O(N), Space Complexity: O(1)) : 3390 ms , Also the most efficient!, Yay! */
    // Edge case: Handle empty slice to avoid returning (255, 0, overflow_error)
    if ages.is_empty() { return (0, 0, 0); }
    
    let mut min_age: u8 = 255;
    let mut max_age: u8 = 0;
    
    for &val in ages
    {
        if val < min_age { min_age = val; }
        if val > max_age { max_age = val; }
    }
    
    (min_age, max_age, max_age-min_age)
    
}