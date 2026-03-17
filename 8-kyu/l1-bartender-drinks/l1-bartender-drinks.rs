fn get_drink_by_profession(param: &str) -> &'static str 
{
    /* My Solution (Time: O(n), Space: O(n)): 1583 ms */
    match param.to_lowercase().as_str()
    {
        "jabroni"           => "Patron Tequila",
        "school counselor"  => "Anything with Alcohol",
        "programmer"        => "Hipster Craft Beer",
        "bike gang member"  => "Moonshine",
        "politician"        => "Your tax dollars",
        "rapper"            => "Cristal",
        _                   => "Beer",
    }
    
    /* The Most Efficient (Time: O(1), Space: O(1)): 1444 ms */
//     if param.eq_ignore_ascii_case("jabroni") { "Patron Tequila" }
//     else if param.eq_ignore_ascii_case("school counselor") { "Anything with Alcohol" }
//     else if param.eq_ignore_ascii_case("programmer") { "Hipster Craft Beer" }
//     else if param.eq_ignore_ascii_case("bike gang member") { "Moonshine" }
//     else if param.eq_ignore_ascii_case("politician") { "Your tax dollars" }
//     else if param.eq_ignore_ascii_case("rapper") { "Cristal" }
//     else { "Beer" }
}