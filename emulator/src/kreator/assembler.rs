use std::fmt;

struct Assembler {
    code: Vec<String>,
    program_counter: usize,
}

impl Assembler {
    fn new(input_code: &str) -> Assembler {
        let mut lines: Vec<String> = Vec::new();
        for line in input_code.split("\n") {
            let line_parts = line.trim().split(";").collect::<Vec<&str>>();
            if line_parts[0].len() != 0 {
                lines.push(String::from(line_parts[0].trim()));
            }
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
        if self.code.len() == 0 as usize {
            return  write!(f, "");
        }
        let mut formatted_code = String::from(&self.code[0]);
        for line in &self.code[1..] {
            formatted_code = format!("{}\n{}", formatted_code, &line);
        }
        write!(f, "{}", formatted_code)
    }
}

impl Iterator for Assembler {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.program_counter += 1;
        if self.program_counter - 1 < self.code.len() {
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
    fn test_display_with_code() {
        let code_file = "MOV A B \n  JMP label \nlabel: INC ACC   ";
        let assembler = Assembler::new(code_file);

        let expected_text = "MOV A B\nJMP label\nlabel: INC ACC";
        assert_eq!(expected_text, format!("{}", assembler));
    }

    #[test]
    fn test_display_without_code() {
        let assembler = Assembler::new("");

        assert_eq!("", format!("{}", assembler));
    }

    #[test]
    fn test_display_remove_comments() {
        let code_file = " \n;comment\nMOV A B ;comment";
        let assembler = Assembler::new(code_file);

        let expected_text = "MOV A B";
        assert_eq!(expected_text, format!("{}", assembler));
    }

    #[test]
    fn test_iterator() {
        let code_file = "MOV A B \n INC ACC";
        let mut assembler = Assembler::new(code_file);

        assert_eq!(Some(0b000000000), assembler.next());
        assert_eq!(Some(0b000000000), assembler.next());
        assert_eq!(None, assembler.next());
    }
}
