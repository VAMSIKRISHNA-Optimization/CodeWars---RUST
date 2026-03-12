                a.0.cmp(&b.0)
            })
        })
    });
    //println!("{:?}", unique_data);
    
    // 6. Formulate the string
    unique_data.into_iter()
        .map(|(ch, v1, v2)| 
        {
            // 1. Determine the prefix based on which value is higher
            let prefix = if v1 > v2 
            {
                "1:"
            } 
            else if v2 > v1 
            {
                "2:"
            } else 
            {
                "=:"
            };
​
            // 2. Find the count to repeat (the maximum of the two)
            let count = v1.max(v2) as usize;
​
            // 3. Format the segment: prefix + repeated character
            format!("{}{}", prefix, ch.to_string().repeat(count))
        })
        .collect::<Vec<String>>()
        .join("/") // 4. Join all segments with a forward slash
​
}