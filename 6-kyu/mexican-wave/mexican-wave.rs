             else
             {
                 ind += 1;
                 SubString.extend(c.to_lowercase());
             }
        });
        
        ans.push(SubString);
    });
    
    // 4. Return the solution
    ans
    
    
    /* The Most Efficient Solution (Time Complexity: O(N*M), Space Complexity: O(N*M)) : 3009 ms */
//     s.char_indices()
//     .filter(|(_, c)| c.is_alphabetic())
//     .map(|(i, _)| {
//         let mut w = s.to_string();
//         // Remove the char at index i and insert its uppercase version
//         let c = w.remove(i);
//         for upc in c.to_uppercase() {
//             w.insert(i, upc);
//         }
//         w
//     })
//     .collect()
    
    
}