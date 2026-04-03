fn parts_sums(ls: &[u64]) -> Vec<u64> 
{
    /* MY SOLUTION ( Time: O(N) , Space: O (N) ): 1677 ms*/ 
    let mut ans: Vec<u64> = Vec::new();
    let mut sum = ls.iter().sum::<u64>();
    ans.push(sum);
    
    (0..ls.len())
    .for_each(|l| 
    {
        sum -= ls[l];
        ans.push(sum);
    });
    
    ans
    
//     /* THE MOST EFFICIENT SOLUTION ( Time: O(N) , Space: O(N) ): 4081 ms*/ 
//     let n = ls.len();
//     // Pre-allocate the exact size needed (N + 1)
//     let mut ans = vec![0; n + 1];
//     let mut current_sum = 0;
​
//     // Fill from right to left to avoid repeated subtractions from a large total
//     for i in (0..n).rev() {
//         current_sum += ls[i];
//         ans[i] = current_sum;
//     }
    
//     // ans[n] is already 0 from initialization
//     ans
}
​