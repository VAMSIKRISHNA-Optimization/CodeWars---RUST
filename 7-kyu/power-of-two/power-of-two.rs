fn power_of_two(mut x: u64) -> bool 
{
    // My Solution : 1497 ms
    if x%2 != 0 || x == 0 { return false; }
    if x == 1 { return true; }
    
    while x > 1
    {
        
        if x%2 != 0 { return false; }
        x /= 2;
    }
    true
    
    // The Most Efficient Way (O(1) Constant Time) : 1364 ms
//     x > 0 && (x & (x - 1)) == 0
    
    // The "Rust Way": 1982 ms
//  x.is_power_of_two()
    
}
​