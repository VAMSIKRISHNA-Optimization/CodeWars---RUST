fn solve(s: &str, mut k: usize) -> String 
{
    /* MY SOLUTION ( Time: O(K x N) , Space: O (N) ): 2766 ms*/ 
    if s.is_empty() { return "".to_string(); }
    
    let mut ns = s.to_string();
    
    while k > 0 && ns.len() > 0
    {
        let ch = ns.bytes().min().unwrap() as char;
        
        let mut removed = false;
        ns = ns.chars()
                  .filter(|&c| 
                  {
                    if c == ch && !removed 
                    {
                        removed = true;
                        false // Filter out (remove) this character
                    } 
                    else 
                    {
                        true  // Keep all other characters
                    }
                })
                .collect::<String>();
        
        k -= 1;
        
    }
    ns
    
//     /* THE MOST EFFICIENT SOLUTION ( Time: O(N) , Space: O(1) ): 1953 ms*/ 
//     if s.is_empty() || k >= s.len() {
//         return String::new();
//     }
​
//     // Step 1: Count frequencies of each character ('a' to 'z')
//     let mut counts = [0; 26];
//     for b in s.bytes() {
//         counts[(b - b'a') as usize] += 1;
//     }
​
//     // Step 2: Determine exactly which characters need to be deleted
//     // We use `k` to "consume" the budgets of the smallest letters first
//     let mut delete_threshold = [0; 26];
//     for i in 0..26 {
//         if k >= counts[i] {
//             k -= counts[i];
//             delete_threshold[i] = counts[i]; // Delete all of them
//         } else {