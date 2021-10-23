use core::fmt;
use regex::Regex;
use std::{collections::HashMap, str::SplitAsciiWhitespace};

const LABEL_DECL: &str = r"^( *[a-zA-Z@?][a-zA-Z@?0-9]{1,4}:)";
const LABEL_USAGE: &str = r"[a-zA-Z@?][a-zA-Z@?0-9]{1,4}";

struct Assembler {
    code: Vec<String>,
}

impl Assembler {
    fn new(input_code: &str) -> Self {
        let mut lines = Vec::new();
        let comment_regex = Regex::new(r";.*").unwrap();

        for line in input_code.split("\n") {
            let line = comment_regex.replace(line, "");
            let line = line.trim_end();
            lines.push(String::from(line));
        }
        
        Self { 
            code: lines,
        }
    }

    fn assemble(&self) -> Result<Vec<u8>, &'static str> {
        let label_regex = Regex::new(LABEL_DECL).unwrap();
        let mut machine_code = Vec::new();

        
        for line in &self.code {
            let line = label_regex.replace(line, "").to_string();
            let line = String::from(line.trim());
            if !line.is_empty() {
                machine_code.extend(to_machine_code(line)?);
            }
        }
        Ok(machine_code)
    }

    fn get_labels(&self) -> Result<HashMap<String, u16>, &'static str> {
        let label_regex = Regex::new(LABEL_DECL).unwrap();

        let mut temp_labels = Vec::new();
        let mut labels = HashMap::new();
        let mut mem_address = 0;
        for line in &self.code {
            if label_regex.is_match(&line) {
                let split = line.split(":").collect::<Vec<&str>>();
                temp_labels.push(String::from(split[0].trim_start()));
                if !split[1].trim().is_empty() {
                    while let Some(new_label) = temp_labels.pop() {
                        if labels.contains_key(&new_label) {
                            return Err("label must not be assigned twice");
                        } else {
                            labels.insert(String::from(new_label), mem_address as u16);
                        }
                    }
                    mem_address += 1;
                }
            } else {
                while let Some(new_label) = temp_labels.pop() {
                    if labels.contains_key(&new_label) {
                        return Err("label must not be assigned twice!");
                    } else {
                        labels.insert(String::from(new_label), mem_address as u16);
                    }
                }
                mem_address += 1;
            }
        }
        if !temp_labels.is_empty() {
            return Err("labels must not point to an empty address!");
        }
        Ok(labels)
    }
}

fn to_machine_code(instruction: String) -> Result<Vec<u8>, &'static str> {
    let label_regex = Regex::new(LABEL_DECL).unwrap();

    let instruction = label_regex.replace(&instruction, "").to_string();
    match instruction.trim_start().split_once(" ") {
        Some((opcode, suffix)) => {
            let arg_binding = suffix.replace(",", " ");
            let args: Vec<&str>  = arg_binding.split_ascii_whitespace().collect();
            match opcode {
                "MOV" => {
                    return convert_mov(args);
                },
                _ => return Err("Could not match instruction"),
            }
        },
        None => {
            match instruction.trim() {
                "NOP" => {
                    return Ok(vec![0x0]);
                },
                "RLC" => {
                    return Ok(vec![0x7]);
                }
                _ => return Err("Could not match instruction"),
            }
        }
    };
}

fn convert_mov(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    let mov_first_argument_err_message = "Invalid first argument for MOV instruction";
    let mov_second_argument_err_message = "Invalid second argument for MOV instruction";
    let mov_missing_argument = "Missing argument(s) for MOV instruction";
    let mov_too_many_arguments = "MOV only takes 2 arguments!";
    let base_value = 0x40;
    let registers = "BCDEHLMA";

    match args.len() {
        0 => return Err(mov_missing_argument),
        1 => return Err(mov_missing_argument),
        2 => { 
            match registers.find(args[0]) {
                Some(index) => {
                    match registers.find(args[1]) {
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
        }
        _ => return Err(mov_too_many_arguments),
    }
}

impl fmt::Display for Assembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use crate::kreator::assembler;

    use super::*;
    use std::collections::HashMap;
    use std::io::{self, BufRead};
    use std::fs::File;

    const OPCODE_TEST_DATA: &str = "./test_data/test_input";

    #[test]
    fn test_display_with_code() {
        let code_file = "MOV A B \n JMP label \nlabel: INC ACC   ";
        let assembler = Assembler::new(code_file);

        let expected_text = "MOV A B\n JMP label\nlabel: INC ACC";
        assert_eq!(expected_text, format!("{}", assembler));
    }

    #[test]
    fn test_display_windows_newline() {
        let code_file = "MOV A B \r\n JMP label \r\nlabel: INC ACC  ";
        let assembler = Assembler::new(code_file);

        let expected_text = "MOV A B\n JMP label\nlabel: INC ACC";
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

        let expected_text = "\n\nMOV A B\n";
        assert_eq!(expected_text, format!("{}", assembler));
    }

    #[test]
    fn test_empty_code_file() {
        let assembler = Assembler::new("");

        assert_eq!(0, assembler.assemble().unwrap().len());
    }

    #[test]
    fn test_rlc() {
        let assembler = Assembler::new("RLC");

        assert_eq!(vec![0x7], assembler.assemble().unwrap());
    }

    #[test]
    fn test_mov_operations() -> io::Result<()> {
        let input_codes = get_instructions_by_opcdoe("MOV").unwrap();

        for line in input_codes {
            let line_components: Vec<&str> = line.split(":").collect();
            let assembler = Assembler::new(line.split(":").collect::<Vec<&str>>()[1]);
            assert_eq!(line_components[0].parse::<u8>().unwrap(), assembler.assemble().unwrap()[0]);
        }
        Ok(())
    }
    
    #[test]
    fn test_mov_errors() {
        let assembler = Assembler::new("MOV A");
        assert_eq!(Err("Missing argument(s) for MOV instruction"), assembler.assemble());

        let assembler = Assembler::new("MOV B,Q");
        assert_eq!(Err("Invalid second argument for MOV instruction"), assembler.assemble());

        let assembler = Assembler::new("MOV M,M");
        assert_eq!(Err("Invalid arguments for MOV instruction (Can't move M into M)"), assembler.assemble());

        let assembler = Assembler::new("MOV A,B,C");
        assert_eq!(Err("MOV only takes 2 arguments!"), assembler.assemble());
    }

    #[test]
    fn test_nop_operation() {
        let assembler = Assembler::new("NOP");
        assert_eq!(0x0, assembler.assemble().unwrap()[0]);

        let assembler = Assembler::new("NOP A");
        assert_eq!(Err("Could not match instruction"), assembler.assemble());
    }

    #[test]
    fn test_invalid_instructions() {
        let assembler = Assembler::new("TEST");

        assert_eq!(Err("Could not match instruction"), assembler.assemble());
    }

    #[test]
    fn test_remove_label_declarations() {
        let input_code = "label: \n MOV A,B\n @LAB:\ntest:\nMOV A,B";
        let assembler = Assembler::new(input_code);

        let mut labels = HashMap::new();
        labels.insert(String::from("test"), 1);
        labels.insert(String::from("@LAB"), 1);
        labels.insert(String::from("label"), 0);

        assert_eq!(labels, assembler.get_labels().unwrap());
        assert_eq!(vec![0x78, 0x78], assembler.assemble().unwrap());
    }

    #[test]
    fn test_duplicate_labels() {
        let assembler = Assembler::new("label:\nlabel:");
        assert_eq!(Err("labels must not point to an empty address!"), assembler.get_labels());

        let assembler = Assembler::new("label:\nlabel:\nMOV A,B");
        assert_eq!(Err("label must not be assigned twice!"), assembler.get_labels());
    }

    fn get_instructions_by_opcdoe(opcode: &str) -> io::Result<Vec<String>> {
        let f = File::open(OPCODE_TEST_DATA)?;
        let mut lines = io::BufReader::new(f).lines();
        let mut instructions = Vec::new();
        
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            if line.contains(opcode) {
                instructions.push(String::from(line));
            }
        }

        Ok(instructions)
    }
}
