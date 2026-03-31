        }); 
    
    Mappings_Vec
    .into_iter()
    .map(|c| (c, Mappings[&c]))
    .collect::<Vec<(char, i32)>>()
    
    /* The Most Efficient Solution (Time Complexity: O(N), Space Complexity: O(K)) : 1558 ms */
    // Pre-allocate capacity to reduce expensive memory re-allocations
//     let mut counts = HashMap::with_capacity(sip.len().min(256));
//     let mut order = Vec::with_capacity(sip.len().min(256));
​
//     for c in sip.chars() {
//         // The Entry API performs exactly ONE lookup for both checking and inserting
//         match counts.entry(c) {
//             Entry::Vacant(e) => {
//                 e.insert(1);
//                 order.push(c); // First appearance, so add to order list
//             }
//             Entry::Occupied(mut e) => {
//                 *e.get_mut() += 1;
//             }
//         }
//     }
​
//     // Convert to the final Vec using the tracked 'order'
//     order.into_iter()
//         .map(|c| (c, counts[&c]))
//         .collect()
}