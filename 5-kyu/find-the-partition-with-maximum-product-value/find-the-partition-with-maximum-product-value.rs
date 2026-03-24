 if !new_counts_extra.is_empty() 
 {
    if best_combos_upd[0] != new_counts_extra
    {
        println!("here");
        best_combos_upd.insert(0, new_counts_extra);
    }
 }
 
 
  let mut best_combos_upd_new :Vec<Vec<u32>> = Vec::new();
 for  vecs in best_combos_upd.clone()
 {
    let max_product: u32 = vecs.iter().product();
    
    if max_product == max_prod_upd
    {
        best_combos_upd_new.push(vecs); 
    }
    
 }
 
​
​
 if max_prod > max_prod_upd
 {
      return (best_combos, max_prod);
 }
 else
 {
     return (best_combos_upd_new, max_prod_upd);
 }
 
}