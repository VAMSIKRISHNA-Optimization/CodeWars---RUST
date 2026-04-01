fn rental_car_cost(d: u32) -> u32 
{
  match d
    {
        d if d < 3          => return (d*40),
        d if d > 3 && d < 7 => return (d*40) - 20,
        d if d >= 7         => return (d*40) - 50, 
        _                   => return 0,
    }
}