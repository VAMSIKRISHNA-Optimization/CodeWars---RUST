use::std::collections::HashMap;
​
fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> 
{
    let mut value_count = HashMap::new();
    
     lst.iter()
        .cloned() 
        .filter(|&x| {
            let count = value_count.entry(x).or_insert(0);
            if *count < n {
                *count += 1;
                true 
            } else {
                false 
            }
        })
        .collect()
    
}