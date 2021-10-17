use std::fmt;

struct Assembler {
    code: Vec<String>,
    program_counter: usize,
}

impl Assembler {
    fn new(input_code: &str) -> Assembler {
        let mut lines = Vec::new();
        for line in input_code.split(" ") {
            lines.push(String::from(line.trim()));
        }
        Assembler { 
            code: lines,
            program_counter: 0,
        }
    }
}

fn to_binary(instruction: &String) -> u8 {
    // TO-DO: return instruction
    0b00000000
}

impl fmt::Display for Assembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted_code = String::from("");
        for (index, line) in self.code.iter().enumerate() {
            if index == self.program_counter {
                formatted_code = format!("{}\n-> {}", formatted_code, &line);
            } else {
                formatted_code = format!("{}\n   {}", formatted_code, &line);
            }
        }
        write!(f, "{}", formatted_code)
    }
}

impl Iterator for Assembler {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.program_counter += 1;
        if self.program_counter < self.code.len() {
            Some(to_binary(&self.code[self.program_counter - 1]))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Assembler;

    #[test]
    fn test_display() {
        let code_file = "MOV A B \n  JMP label \nlabel: INC ACC   ";
        let ass = Assembler::new(code_file);
    }
}
