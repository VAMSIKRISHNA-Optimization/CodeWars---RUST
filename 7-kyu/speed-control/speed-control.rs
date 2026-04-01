fn gps(s: i32, x: Vec<f64>) -> i32 
{
    /* My Solution (Time Complexity: O(N), Space Complexity: O(N)) : 1277 ms */
    x
    .windows(2)
    .map(|w| ((3600_f64*(w[1]-w[0]))/s as f64).floor() as i32 )
    .collect::<Vec<i32>>()
    .into_iter()
    .max()
    .unwrap_or(0) 
    
    /* The Most Efficient Solution (Time Complexity: O(N+M), Space Complexity: O(1)) : 1253 ms */
//     if x.len() < 2 { return 0; }
​
//     x.windows(2)
//         .map(|w| (3600.0 * (w[1] - w[0]) / s as f64) as i32)
//         .max()           // Finds max on the fly
//         .unwrap_or(0)
​
}