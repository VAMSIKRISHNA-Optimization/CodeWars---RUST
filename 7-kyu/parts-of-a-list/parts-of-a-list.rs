fn part_list(arr: Vec<&str>) -> String 
{
    (1..arr.len())
    .map(|i| 
        {
            let (first, last) = arr.split_at(i);
            format!("({}, {})", first.join(" "),last.join(" "))
        })
    .collect::<Vec<String>>()
    .join("")
}