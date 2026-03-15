fn greet(name: &str) -> String 
{
    let new_name = name
        .chars()
        .enumerate()
        .map(|(i,c)| 
        {
            if i==0 
            { 
                c.to_uppercase().to_string() 
                
            } 
            else 
            { 
                c.to_lowercase().to_string() 
                
            }
        })
        .collect::<String>();
    
    format!("Hello {}!", new_name)
}