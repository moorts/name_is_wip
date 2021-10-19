use core::fmt;
use regex::Regex;

struct Assembler {
    code: Vec<String>,
}

impl Assembler {
    fn new(input_code: &str) -> Self {
        let mut lines = Vec::new();
        let comment_regex = Regex::new(r";.*").unwrap();

        for line in input_code.split("\n") {
            let line = comment_regex.replace(line, "");
            let line = line.trim();
            if !line.is_empty() {
                lines.push(String::from(line));
            }
        }
        Self { 
            code: lines,
        }
    }

    fn get_machine_code(&self) -> Result<Vec<u8>, &'static str> {
        let mut machine_code = Vec::new();
        for line in &self.code {
            machine_code.extend(to_machine_code(line)?);
        }
        Ok(machine_code)
    }
}

fn to_machine_code(instruction: &String) -> Result<Vec<u8>, &'static str> {
    let mut machine_code = Vec::new();
    let label_regex = Regex::new(r"^([a-zA-Z@?][a-zA-Z@?0-9]{1,4}:)").unwrap();
    let mov_regex = Regex::new(r"([A-Z] *, *[A-Z])$").unwrap();

    let formatted_instruction = instruction.replace(",", " ");
    let instruction_fields: Vec<&str> = formatted_instruction.split_ascii_whitespace().collect();

    let mut first_index = 0;
    if label_regex.is_match(&instruction) {
        first_index = 1;
    }

    match instruction_fields[first_index] {
        "NOP" => machine_code.push(0x0),
        "MOV" => {
            let mov_first_argument_err_message = "Invalid first argument for MOV instruction";
            let mov_second_argument_err_message = "Invalid second argument for MOV instruction";
            if mov_regex.is_match(instruction) {
                match instruction_fields[first_index + 1] {
                    "B" => {
                        match instruction_fields[first_index + 2] {
                            "B" => machine_code.push(0x40),
                            "C" => machine_code.push(0x41),
                            "D" => machine_code.push(0x42),
                            "E" => machine_code.push(0x43),
                            "H" => machine_code.push(0x44),
                            "L" => machine_code.push(0x45),
                            "M" => machine_code.push(0x46),
                            "A" => machine_code.push(0x47),
                            _ => return Err(mov_second_argument_err_message),
                        }
                    },
                    "C" => {
                        match instruction_fields[first_index + 2] {
                            "B" => machine_code.push(0x48),
                            "C" => machine_code.push(0x49),
                            "D" => machine_code.push(0x4a),
                            "E" => machine_code.push(0x4b),
                            "H" => machine_code.push(0x4c),
                            "L" => machine_code.push(0x4d),
                            "M" => machine_code.push(0x4e),
                            "A" => machine_code.push(0x4f),
                            _ => return Err(mov_second_argument_err_message),
                        }
                    },
                    "D" => {
                        match instruction_fields[first_index + 2] {
                            "B" => machine_code.push(0x50),
                            "C" => machine_code.push(0x51),
                            "D" => machine_code.push(0x52),
                            "E" => machine_code.push(0x53),
                            "H" => machine_code.push(0x54),
                            "L" => machine_code.push(0x55),
                            "M" => machine_code.push(0x56),
                            "A" => machine_code.push(0x57),
                            _ => return Err(mov_second_argument_err_message),
                        }
                    },
                    "E" => {
                        match instruction_fields[first_index + 2] {
                            "B" => machine_code.push(0x58),
                            "C" => machine_code.push(0x59),
                            "D" => machine_code.push(0x5a),
                            "E" => machine_code.push(0x5b),
                            "H" => machine_code.push(0x5c),
                            "L" => machine_code.push(0x5d),
                            "M" => machine_code.push(0x5e),
                            "A" => machine_code.push(0x5f),
                            _ => return Err(mov_second_argument_err_message),
                        }
                    },
                    "H" => {
                        match instruction_fields[first_index + 2] {
                            "B" => machine_code.push(0x60),
                            "C" => machine_code.push(0x61),
                            "D" => machine_code.push(0x62),
                            "E" => machine_code.push(0x63),
                            "H" => machine_code.push(0x64),
                            "L" => machine_code.push(0x65),
                            "M" => machine_code.push(0x66),
                            "A" => machine_code.push(0x67),
                            _ => return Err(mov_second_argument_err_message),
                        }
                    },
                    "L" => {
                        match instruction_fields[first_index + 2] {
                            "B" => machine_code.push(0x68),
                            "C" => machine_code.push(0x69),
                            "D" => machine_code.push(0x6a),
                            "E" => machine_code.push(0x6b),
                            "H" => machine_code.push(0x6c),
                            "L" => machine_code.push(0x6d),
                            "M" => machine_code.push(0x6e),
                            "A" => machine_code.push(0x6f),
                            _ => return Err(mov_second_argument_err_message),
                        }
                    },
                    "M" => {
                        match instruction_fields[first_index + 2] {
                            "B" => machine_code.push(0x70),
                            "C" => machine_code.push(0x71),
                            "D" => machine_code.push(0x72),
                            "E" => machine_code.push(0x73),
                            "H" => machine_code.push(0x74),
                            "L" => machine_code.push(0x75),
                            "A" => machine_code.push(0x77),
                            _ => return Err(mov_second_argument_err_message),
                        }
                    },
                    "A" => {
                        match instruction_fields[first_index + 2] {
                            "B" => machine_code.push(0x78),
                            "C" => machine_code.push(0x79),
                            "D" => machine_code.push(0x7a),
                            "E" => machine_code.push(0x7b),
                            "H" => machine_code.push(0x7c),
                            "L" => machine_code.push(0x7d),
                            "M" => machine_code.push(0x7e),
                            "A" => machine_code.push(0x7f),
                            _ => return Err(mov_second_argument_err_message),
                        }
                    },
                    _ => return Err(mov_first_argument_err_message),
                }
            } else {
                return Err("Missing argument(s) for MOV instruction");
            }
        }
        _ => return Err("Could not match instruction"),
    }
    Ok(machine_code)
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
    fn test_empty_code_file() {
        let assembler = Assembler::new("");

        assert_eq!(0, assembler.get_machine_code().unwrap().len());
    }

    #[test]
    fn test_mov_operations() {
        let code_file = "MOV A, B \n MOV L  ,M\nMOV B,B";
        let assembler = Assembler::new(code_file);

        let machine_code = assembler.get_machine_code().unwrap();
        assert_eq!(3, machine_code.len());
        assert_eq!(0x78, machine_code[0]);
        assert_eq!(0x6e, machine_code[1]);
        assert_eq!(0x40, machine_code[2]);
    }
    
    #[test]
    fn test_mov_errors() {
        let assembler = Assembler::new("MOV A");
        assert_eq!(Err("Missing argument(s) for MOV instruction"), assembler.get_machine_code());

        let assembler = Assembler::new("MOV B,Q");
        assert_eq!(Err("Invalid second argument for MOV instruction"), assembler.get_machine_code());
    }

    #[test]
    fn test_nop_operation() {
        let assembler = Assembler::new("NOP");

        assert_eq!(0x0, assembler.get_machine_code().unwrap()[0]);
    }

    #[test]
    fn test_invalid_instructions() {
        let assembler = Assembler::new("TEST");

        assert_eq!(Err("Could not match instruction"), assembler.get_machine_code());
    }
}
