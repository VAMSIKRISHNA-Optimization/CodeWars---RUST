fn nearest_sq(n: u32) -> u32 
{
    /* My Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1501 ms */
    let sqrt_val = ((n as f32).sqrt()) as u32;
    if  sqrt_val == n {  return n; }
    
    let lesser_square  = sqrt_val.pow(2);
    let greater_square = (sqrt_val + 1).pow(2);
    
    let l_dif = n - lesser_square;
    let g_dif = greater_square - n;
    
    if g_dif <  l_dif { return greater_square; }
    else { return lesser_square; }
    
    /* The Most Effective Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1457 ms */
//     let root = (n as f64).sqrt().round() as u32;
//     root.pow(2)
    
}