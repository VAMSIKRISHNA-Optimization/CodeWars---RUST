            (5,3) => total_score += 500,
            (4,3) => total_score += 400,
            (3,3) => total_score += 300,
            (2,3) => total_score += 200,
            
            (6,5) => total_score += 600,
            (6,4) => total_score += 600,
            
            (5,5) => total_score += 600,
            (5,4) => total_score += 550,
            
            (4,5) => total_score += 400,
            (4,4) => total_score += 400,
            
            (3,5) => total_score += 300,
            (3,4) => total_score += 300,
            
            (2,5) => total_score += 200,
            (2,4) => total_score += 200,
            
            
            (2,5) => total_score += 200,
            (2,4) => total_score += 200,
            
            (5,2) => total_score += 100,
            (5,1) => total_score += 50,
            
            _ => total_score += 0,
        }
    }
    
    total_score
                  
}