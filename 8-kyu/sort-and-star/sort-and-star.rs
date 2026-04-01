fn two_sort(arr: &[&str]) -> String 
{
    arr
    .iter()
    .min()
    .map(|ss| 
        ss.chars()
        .map(|c| 
            c.to_string())
        .collect::<Vec<String>>()
        .join("***"))
    .unwrap()
}