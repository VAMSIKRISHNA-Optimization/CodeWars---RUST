//     let sorted_set: BTreeSet<usize> = ids.iter().copied().collect();
​
    
//     if sorted_set.len() == 1 
//     {
//         if *sorted_set.first().unwrap() == 0 { return 1; }
//         else if *sorted_set.first().unwrap() == 1 { return 0; }
//         else { return sorted_set.first().unwrap() - 1; }
//     }
    
//     let diffs: Vec<usize> = sorted_set.iter()
//                                     .zip(sorted_set.iter().skip(1))
//                                     .map(|(a, b)| b - a)
//                                     .collect();
    
//     if let Some(1) = diffs.iter().max()
//     {
//         if *sorted_set.first().unwrap() == 0 { return 1; }
//         else if *sorted_set.first().unwrap() == 1 { return 0; }
//         else { return sorted_set.first().unwrap() - 1; }
//     }
//     else
//     {
//         sorted_set
//         .iter()
//         .zip(sorted_set.iter().skip(1))
//         .max_by_key(|&(a, b)| b - a)
//         .map(|(a, _)| *a) 
//         .unwrap() + 1
​
//     }
​
// }