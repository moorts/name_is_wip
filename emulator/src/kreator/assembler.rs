use std::fmt;

struct Assembler {
    code: Vec<String>,
    pc: usize,
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
            pc: 0,
        }
    }
}

fn to_machine_code(instruction: &String) -> Vec<u8> {
    // TO-DO: return instruction
    Vec::from([0b00000000])
}

impl fmt::Display for Assembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.join("\n"))
    }
}

impl Iterator for Assembler {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.pc += 1;
        if self.pc - 1 < self.code.len() {
            Some(to_machine_code(&self.code[(self.pc - 1) as usize]))
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

    fn test_display_windows_newline() {
        let code_file = "MOV A B \r\n JMP label \r\nlabel: INC ACC  ";
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


        // TO-DO: replace Some(0) with the actual statements that should be returned
        assert_eq!(Some(Vec::from([0b000000000])), assembler.next());
        assert_eq!(Some(Vec::from([0b000000000])), assembler.next());
        assert_eq!(None, assembler.next());
    }

    #[test]
    fn test_empty_iterator() {
        let mut assembler = Assembler::new("");

        assert_eq!(None, assembler.next());
    }
}
