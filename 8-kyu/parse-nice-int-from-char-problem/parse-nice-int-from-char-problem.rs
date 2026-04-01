fn get_age(age: &str) -> u32 
{
  age.chars().next().unwrap().to_digit(10).expect("Not a NUMBER in 0-9")
}