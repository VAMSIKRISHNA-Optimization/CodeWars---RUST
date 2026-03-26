use itertools::Itertools;
fn same_length(txt: &str) -> bool 
{
    if txt.len() % 2 == 1 { return false;}
    if txt.chars().filter(|&c| c == '1').count() != txt.chars().filter(|&c| c == '0').count() { return false; }
​
    
    txt.chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(_, group)| group.collect())
        .collect::<Vec<String>>()
        .chunks(2)
        .all(|pair|
        {
            let s1 = &pair[0];
            let s2 = &pair[1];
            s1.len() == s2.len() && s1.starts_with('1') && s2.starts_with('0')
        })
}