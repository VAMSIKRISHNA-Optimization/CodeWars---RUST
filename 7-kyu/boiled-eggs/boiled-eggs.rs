fn cooking_time(eggs: u32) -> u32 
{
    /* My Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1090 ms */
    if eggs == 0        { return 0; }
    else if eggs <= 8   { return 5; } 
    else
    {
        return ((eggs/8) * 5) + ((eggs % 8 > 0) as u32 * 5);
    }
    
    /* The Most Efficient Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1020 ms */
    // Standard ceiling division formula: (n + (k-1)) / k
    // Then multiply by the 5 minutes per batch.
//     ((eggs + 7) / 8) * 5
}