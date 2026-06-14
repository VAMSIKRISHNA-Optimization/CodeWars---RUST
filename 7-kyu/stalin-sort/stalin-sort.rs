fn stalin_sort(vector: &mut Vec<i32>) 
{
    /* MY SOLUTION ( Time: O(N) , Space: O (1) ): 1177 ms*/ 
    // Also, the most efficient
    if vector.is_empty() || vector.len() == 1 { return; }
    let mut last     = vector[0];
    let mut is_first = true;
    
    vector.retain(|&x| 
    {
        if is_first { is_first = false; return true; }
        if x >= last 
        {
            last = x;
            true
        }
        else
        {
            false
        }
    });
}