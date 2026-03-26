fn how_much_i_love_you(nb_petals: u16) -> &'static str 
{
    if nb_petals%6 == 0 {"not at all"}
    else
    {    
        match nb_petals % 6
        {
            1 => "I love you",
            2 => "a little",
            3 => "a lot",
            4 => "passionately",
            5 => "madly",
            _ => panic!("FAILED")
        } 
    }
​
}