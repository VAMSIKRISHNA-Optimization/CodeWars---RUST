fn find_even_index(arr: &[i32]) -> Option<usize> 
{
//     if arr.iter().all(|&x| x == 0) { return Some(0 as usize); }
//     if arr.len() == 1 { return Some(0 as usize); }
//     if arr.iter().sum::<i32>() == 0 && arr.len() > 1 { return None; }
//     if arr.is_empty() { return None; }
 
    
    arr.iter().enumerate().position(|(ind,_)| 
                            {
                                let sum_left:  i32 = arr[..ind].iter().sum::<i32>();
                                let sum_right: i32 = arr[ind+1..].iter().sum::<i32>();
                                sum_left == sum_right
                            })
    
}