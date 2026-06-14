fn minimum_perimeter(area: u64) -> u64 
{
   /* MY SOLUTION ( Time: O(sqrt(N)) , Space: O (sqrt(N)) ): 1698 ms*/ 
   let all_pairs = get_inner_factor_pairs(area);
   if all_pairs.is_empty() { return 2 * (1 + area); }
   
   let min_pair = all_pairs
                  .iter()
                  .min_by_key(|&(l,b)| l+b)
                  .unwrap();
                   
    2 * (min_pair.0 + min_pair.1)
    
//     /* THE MOST EFFICIENT SOLUTION ( Time: O(sqrt(N)) , Space: O(1) ): 1050ms*/ 
//     // Start scanning from the square root downwards
//     let mut width = (area as f64).sqrt() as u64;
    
//     while width > 0 {
//         if area % width == 0 {
//             let length = area / width;
//             // The first factor pair found is the closest to a perfect square,
//             // which automatically minimizes the perimeter.
//             return 2 * (length + width);
//         }
//         width -= 1;
//     }
    
//     2 * (1 + area) // Fallback safety clause
}
​
fn get_inner_factor_pairs(num: u64) -> Vec<(u64, u64)> 
{
    if num < 4 { return vec![]; }
​
    let mut pairs = Vec::new();
    let limit = (num as f64).sqrt() as u64;
​
    for i in 2..=limit 
    {
        if num % i == 0 
        {
            pairs.push((i, num / i));
        }
    }
​
    pairs
}
​