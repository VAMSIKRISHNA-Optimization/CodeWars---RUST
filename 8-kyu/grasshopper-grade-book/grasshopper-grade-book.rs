fn get_grade(s1: u16, s2: u16, s3: u16) -> char 
{
    match (s1+s2+s3)/3
    {
        a if a >= 90 && a <= 100 => 'A',
        b if b >= 80 && b <= 90  => 'B',
        c if c >= 70 && c <= 80  => 'C',
        d if d >= 60 && d <= 70  => 'D',
        _ => 'F',
        
    }
}