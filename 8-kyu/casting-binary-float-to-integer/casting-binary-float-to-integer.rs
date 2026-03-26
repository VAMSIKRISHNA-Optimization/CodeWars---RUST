pub fn convert_to_i32(f: f32) -> i32 
{
    /* My Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1172 ms */
    let f32_binary_string = format!("{:032b}", f.to_bits());
    i32::from_str_radix(&f32_binary_string, 2).unwrap()
    
    /* The Most Effective Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1129 ms */
//     f.to_bits() as i32
}