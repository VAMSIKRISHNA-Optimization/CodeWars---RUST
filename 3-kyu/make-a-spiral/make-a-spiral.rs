                else if Point.0 > up_end
                {
                    Cur_Momentum = Momentum::Up;
                    Point.0 = Point.0-1;
                    solution[Point.0][Point.1] = 1;
                }
            }
            
            Momentum::Nope => break,
            _ => break,
        }
        
    }
    if size % 2 == 0
    {
        solution[Point.0][Point.1] = 0;
    }
  
    
    for vec in &solution
    {
        for val in vec
        {
            print!("{:?}", val);
        }
        print!("\n");
    }
​
    
    
    
    solution
}