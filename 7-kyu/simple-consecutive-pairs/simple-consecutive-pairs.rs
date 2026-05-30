fn pairs(arr: &[i32]) -> usize 
{
     /* My Solution : (Time Complexity: O(N), Space Complexity: O(1)) #Time: 1146 ms */   
    arr.chunks(2).fold(0 as usize, |acc, ch| 
    {
        if ch.len() == 2 && (ch[1] - ch[0]).abs() == 1 
        {
            acc + 1
        } else {
            acc
        }
    })
    
//     /* The Most Efficient Solution : (Time Complexity: O(N), Space Complexity: O(1)) #Time: 1260 ms */
//         arr.chunks(2).fold(0, |acc, ch| {
//             // Condition evaluates to true (1) or false (0), avoiding the if/else branch
//             let is_pair = (ch.len() == 2 && (ch[1] - ch[0]).abs() == 1) as usize;
//             acc + is_pair
//         })
}
    