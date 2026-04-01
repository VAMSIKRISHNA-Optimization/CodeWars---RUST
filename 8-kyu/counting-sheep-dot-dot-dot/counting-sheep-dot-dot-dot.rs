fn count_sheep(sheep: &[bool]) -> u8 
{
    sheep.iter().fold(0, |acc,&v| { if v {acc+1} else {acc}}) as u8
}