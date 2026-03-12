//     while factr < val
//     {
//         if val % factr == 0 { return false; }
//         factr += 1;
//     }
//     return true;
// }
​
// fn prime_power(val: i64) -> (Option<i64>, Option<u32>)
// {
//     let mut base_val: i64 = 2;
//     let mut pow_val : u32 = 1;
    
//     while base_val <= val
//     {
//         if is_prime_num(base_val)
//         {
//             pow_val = 1;
//             while base_val.pow(pow_val) <= val
//             {
//                 if base_val.pow(pow_val) == val
//                 {
//                     return ( Some(base_val), Some(pow_val) );
//                 }
//                 pow_val += 1;
//             }
//         }
//         base_val += 1;
//     }
    
//     return (None, None);
// }