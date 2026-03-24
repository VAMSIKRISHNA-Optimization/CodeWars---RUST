fn pillars(num_of_pillars: u32, distance: u32, width: u32) -> u32 
{
    /* My Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1241 ms (Also, the most efficient solution, YaY!) */
    if num_of_pillars == 1 { return 0; }
    ((num_of_pillars-1) * distance * 100) + ((num_of_pillars-2) * width)
}