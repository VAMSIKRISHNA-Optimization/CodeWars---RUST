use std::cmp;
fn rgb(r: i32, g: i32, b: i32) -> String 
{
  let r_bounded = cmp::max(cmp::min(r,255),0);
  let g_bounded = cmp::max(cmp::min(g,255),0);
  let b_bounded = cmp::max(cmp::min(b,255),0);
​
  let r_string_upper = format!("{:02X}", r_bounded);
  let g_string_upper = format!("{:02X}", g_bounded);
  let b_string_upper = format!("{:02X}", b_bounded);
  
  let result = format!("{}{}{}", r_string_upper, g_string_upper, b_string_upper);
  
  return result;   
  
}