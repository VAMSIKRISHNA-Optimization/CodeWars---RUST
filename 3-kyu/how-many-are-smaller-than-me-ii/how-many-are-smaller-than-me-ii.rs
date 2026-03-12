​
fn smaller(arr: &[i32]) -> Vec<usize> 
{   
    
    if arr.is_empty() 
    { 
        return Vec::from([]);
    }
    
    let mut input_arr_vec = Vec::from(arr);
    input_arr_vec.sort_unstable();
    input_arr_vec.dedup();
    
    let mut smaller_count: Vec<usize> = vec![0; arr.len()];
    let mut FenwickTree  : Vec<i32>   = vec![0; input_arr_vec.len()];
​
​
    for (ind, &value) in arr.iter().enumerate().rev() 
    {
        let freq = input_arr_vec.binary_search(&value).unwrap() + 1;
        
        
        let mut qfreq = freq - 1;
        let mut count = 0;
        while qfreq > 0
        {
            count += FenwickTree[qfreq];
            qfreq -= (qfreq as isize & -(qfreq as isize)) as usize;
        }
        smaller_count[ind] = count as usize;
​
        let mut nfreq = freq;
        while nfreq < FenwickTree.len()
        {
            FenwickTree[nfreq] += 1;
            nfreq += (nfreq as isize & -(nfreq as isize)) as usize;
        }
    }
​
    smaller_count
}