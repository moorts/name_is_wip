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
    let label_regex = Regex::new(r"^([a-zA-Z@?][a-zA-Z@?0-9]{1,4}:)").unwrap();
    let mov_regex = Regex::new(r"([A-Z] *, *[A-Z])$").unwrap();

    let formatted_instruction = instruction.replace(",", " ");
    let instruction_fields: Vec<&str> = formatted_instruction.split_ascii_whitespace().collect();

    let mut opcode_index = 0;
    if label_regex.is_match(&instruction) {
        opcode_index = 1;
    }

    match instruction_fields[opcode_index] {
        "NOP" => {
            if instruction_fields.len() - opcode_index > 1 {
                return Err("NOP does not take any arguments!");
            }
            return Ok(vec![0x0]);
        },
        "MOV" => {
            if instruction_fields.len() - opcode_index > 3 {
                return Err("MOV only takes 2 arguments!");
            }
            let mov_first_argument_err_message = "Invalid first argument for MOV instruction";
            let mov_second_argument_err_message = "Invalid second argument for MOV instruction";
            if mov_regex.is_match(instruction) {
                let base_value = 0x40;
                let registers = "BCDEHLMA";
                match registers.find(instruction_fields[opcode_index + 1]) {
                    Some(index) => {
                        match registers.find(instruction_fields[opcode_index + 2]) {
                            Some(second_index) =>  {
                                if index == 6 && second_index == 6 {
                                    return Err("Invalid arguments for MOV instruction (Can't move M into M)");
                                }
                                let instruction_value = base_value + (index as u8 * 8) + second_index as u8;
                                return Ok(vec![instruction_value]);
                            },
                            None => return Err(mov_second_argument_err_message),
                        }
                    },
                    None => return Err(mov_first_argument_err_message),
                }
            } else {
                return Err("Missing argument(s) for MOV instruction");
            }
        },
        _ => return Err("Could not match instruction"),
    }
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
        let code_file = "MOV A, B \n MOV L  ,M\nMOV B,M";
        let assembler = Assembler::new(code_file);

        let machine_code = assembler.get_machine_code().unwrap();
        assert_eq!(3, machine_code.len());
        assert_eq!(0x78, machine_code[0]);
        assert_eq!(0x6e, machine_code[1]);
        assert_eq!(0x46, machine_code[2]);
    }

    #[test]
    fn test_mov_edges() {
        let assembler = Assembler::new("MOV B,B \nMOV M,L\nMOV M,A\n MOV A,A");

        let machine_code = assembler.get_machine_code().unwrap();
        assert_eq!(0x40, machine_code[0]);
        assert_eq!(0x75, machine_code[1]);
        assert_eq!(0x77, machine_code[2]);
        assert_eq!(0x7f, machine_code[3]);
    }
    
    #[test]
    fn test_mov_errors() {
        let assembler = Assembler::new("MOV A");
        assert_eq!(Err("Missing argument(s) for MOV instruction"), assembler.get_machine_code());

        let assembler = Assembler::new("MOV B,Q");
        assert_eq!(Err("Invalid second argument for MOV instruction"), assembler.get_machine_code());

        let assembler = Assembler::new("MOV M,M");
        assert_eq!(Err("Invalid arguments for MOV instruction (Can't move M into M)"), assembler.get_machine_code());

        let assembler = Assembler::new("MOV A,B,C");
        assert_eq!(Err("MOV only takes 2 arguments!"), assembler.get_machine_code());
    }

    #[test]
    fn test_nop_operation() {
        let assembler = Assembler::new("NOP");
        assert_eq!(0x0, assembler.get_machine_code().unwrap()[0]);

        let assembler = Assembler::new("NOP A");
        assert_eq!(Err("NOP does not take any arguments!"), assembler.get_machine_code());
    }

    #[test]
    fn test_invalid_instructions() {
        let assembler = Assembler::new("TEST");

        assert_eq!(Err("Could not match instruction"), assembler.get_machine_code());
    }
}
