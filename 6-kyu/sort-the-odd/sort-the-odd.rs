fn sort_array(arr: &[i32]) -> Vec<i32> 
{
    // 1. Array to vec conversion
    let mut vec_arr = arr.to_vec();
    
    // 2. Isolate the odd numbers 
    let mut odd_vec = arr.iter()
                         .filter(|&x| x % 2 != 0)
                         .copied()
                         .collect::<Vec<i32>>();
    
    // 3. Sort the odd numbers
    odd_vec.sort();
    
    // 4. Re-assign the sorted odd numbers into the vec
    let mut odd_iter = odd_vec.into_iter();
    for val in vec_arr.iter_mut() {
        if *val % 2 != 0 {
            // Replace with the next sorted odd number
            *val = odd_iter.next().unwrap();
        }
    }
    // 5. Return the vec
    vec_arr
    
}