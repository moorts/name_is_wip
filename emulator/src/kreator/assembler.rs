use core::fmt;

use regex::Regex;

struct Assembler {
    code: Vec<String>,
}

impl Assembler {
    fn new(input_code: &str) -> Assembler {
        let mut lines: Vec<String> = Vec::new();
        let comment_regex = Regex::new(r";.*").unwrap();

        for line in input_code.split("\n") {
            let line = comment_regex.replace(line, "");
            let line = line.trim();
            if line.len() != 0 {
                lines.push(String::from(line));
            }
        }
        Assembler { 
            code: lines,
        }
    }

    fn get_machine_code(&self) -> Result<Vec<Vec<u8>>, &'static String> {
        let mut machine_code = Vec::new();
        for line in &self.code {
            match to_machine_code(line) {
                Ok(instruction) => machine_code.push(instruction),
                Err(error) => return Err(error),
            }
        }
        Ok(machine_code)
    }
}

fn to_machine_code(instruction: &String) -> Result<Vec<u8>, &'static String> {
    // TO-DO: return instruction
    Ok(Vec::from([0b00000000]))
}

impl fmt::Display for Assembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.join("\n"))
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
        let code_file = " \n;comment\nMOV A B ;comment\n;";
        let assembler = Assembler::new(code_file);

        let expected_text = "MOV A B";
        assert_eq!(expected_text, format!("{}", assembler));
    }

    #[test]
    fn test_iterator() {
        let code_file = "MOV A B \n INC ACC";
        let assembler = Assembler::new(code_file);

        assert_eq!(2, assembler.get_machine_code().unwrap().len());
    }

    #[test]
    fn test_empty_iterator() {
        let assembler = Assembler::new("");

        assert_eq!(0, assembler.get_machine_code().unwrap().len());
    }
}
