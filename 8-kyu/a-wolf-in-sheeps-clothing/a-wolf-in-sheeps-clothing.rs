fn warn_the_sheep(queue: &[&str]) -> String 
{
    let index = queue.iter().rev().position(|&x| x == "wolf").unwrap();
    
    match index
    {
        0 =>"Pls go away and stop eating my sheep".to_string(),
        _ => format!("Oi! Sheep number {}! You are about to be eaten by a wolf!",index),
    }
}