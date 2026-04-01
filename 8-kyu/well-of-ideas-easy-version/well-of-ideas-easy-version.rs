fn well(x: &[&str]) -> &'static str 
{
    let count = x.iter().filter(|&&s| s == "good").count();
    match count 
    {
        0 => "Fail!",
        1 => "Publish!",
        2 => "Publish!",
        _ => "I smell a series!",
    }
}