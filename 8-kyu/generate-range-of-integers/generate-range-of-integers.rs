fn generate_range(min: usize, max: usize, step: usize) -> Vec<usize> 
{
    // My solution (Time:O(n) , Space: O(n) ) 1285ms (Also, the most efficient one!!! Yay!)
    (min..=max).step_by(step).collect::<Vec<usize>>()
}