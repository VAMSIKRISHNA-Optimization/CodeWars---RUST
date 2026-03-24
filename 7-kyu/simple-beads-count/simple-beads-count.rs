fn count_red_beads(n: u32) -> u32 
{
    // My Solution (Time: O(1), Space: O(1)): 1327 ms - Also, the most efficient (Yay!)
    if n < 2 { return 0; }
    n*2 - 2
}