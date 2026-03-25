macro_rules! even_or_odd 
{
    () => { panic!("Invalid Input");};
    
    ($name: expr) =>
    {
        match ($name).abs()%2
        {
            0 => "Even",
            _ => "Odd",
        }
    };
    
}