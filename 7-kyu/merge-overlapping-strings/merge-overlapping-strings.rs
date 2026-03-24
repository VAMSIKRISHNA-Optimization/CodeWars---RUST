//     let mut pi = vec![0; n];
//     let mut j = 0;
    
//     // 1. Precompute KMP failure function (pi table) for 'second'
//     for i in 1..n {
//         while j > 0 && s_chars[i] != s_chars[j] {
//             j = pi[j - 1];
//         }
//         if s_chars[i] == s_chars[j] {
//             j += 1;
//         }
//         pi[i] = j;
//     }
​
//     // 2. Step through 'first' to find the overlap at the end
//     let mut q = 0; 
//     for &c in &f_chars {
//         // The fix: if q reached the end of 'second', reset using pi table
//         if q == n {
//             q = pi[q - 1];
//         }
        
//         while q > 0 && c != s_chars[q] {
//             q = pi[q - 1];
//         }
//         if c == s_chars[q] {
//             q += 1;
//         }
//     }
​
//     // q is the length of the longest suffix-prefix overlap
//     let result_suffix: String = s_chars.into_iter().skip(q).collect();
//     format!("{}{}", first, result_suffix)
}