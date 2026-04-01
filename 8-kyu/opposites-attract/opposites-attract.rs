fn lovefunc(flower1: u16, flower2: u16) -> bool {
    if flower1 % 2 == 0 && flower2 % 2 == 1{                            
            return true
        }
    else if flower1 % 2 == 1 && flower2 % 2 == 0 {
            return true
        }    
    else{
            return false            
            }
            
    }