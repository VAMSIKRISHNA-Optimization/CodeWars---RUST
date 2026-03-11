fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 
{
    if a1.is_empty() || a2.is_empty() { return -1; }
    
    let min1 = a1.iter().map(|s| s.len()).min().unwrap();
    let max1 = a1.iter().map(|s| s.len()).max().unwrap();
    let min2 = a2.iter().map(|s| s.len()).min().unwrap();
    let max2 = a2.iter().map(|s| s.len()).max().unwrap();
​
    // The max difference is either (max1 - min2) or (max2 - min1)
    std::cmp::max(max1 as i32 - min2 as i32, max2 as i32 - min1 as i32).abs()
    
}