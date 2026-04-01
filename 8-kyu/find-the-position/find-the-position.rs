use::std::collections::HashMap;
​
fn position(c: char) -> String 
{
    let ref_map: HashMap<char, usize> = ('a'..='z')
                                        .enumerate()
                                        .map(|(i, c)| (c, i + 1))
                                        .collect();
    format!("Position of alphabet: {}", ref_map.get(&c).unwrap())
    
}