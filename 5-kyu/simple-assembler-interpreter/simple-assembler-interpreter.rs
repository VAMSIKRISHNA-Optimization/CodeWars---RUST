use std::collections::HashMap;
​
fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    let mut pc: i64 = 0; // Program counter
​
    while pc >= 0 && (pc as usize) < program.len() {
        let line = program[pc as usize];
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = parts[0];
​
        // Helper to get value of an operand (either a constant or a register)
        let get_val = |s: &str, regs: &HashMap<String, i64>| {
            s.parse::<i64>().unwrap_or_else(|_| *regs.get(s).unwrap_or(&0))
        };
​
        match cmd {
            "mov" => {
                let reg = parts[1].to_string();
                let val = get_val(parts[2], &registers);
                registers.insert(reg, val); // Use insert for assignment, not +=
                pc += 1;
            }
            "inc" => {
                *registers.entry(parts[1].to_string()).or_insert(0) += 1;
                pc += 1;
            }
            "dec" => {
                *registers.entry(parts[1].to_string()).or_insert(0) -= 1;
                pc += 1;
            }
            "jnz" => {
                let check = get_val(parts[1], &registers);
                if check != 0 {
                    pc += get_val(parts[2], &registers);
                } else {
                    pc += 1;
                }
            }
            _ => pc += 1,
        }
    }
    registers
}