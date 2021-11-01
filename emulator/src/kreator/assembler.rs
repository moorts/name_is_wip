use core::fmt;
use regex::Regex;
use std::{collections::HashMap};

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
                "MOV" => return convert_mov_args(args),
                "STAX" => return convert_stax_args(args),
                "INX" => return convert_inx_args(args),
                "INR" => return convert_inr_args(args),
                "DCR" => return convert_dcr_args(args),
                _ => return Err("Could not match instruction"),
            }
        },
        None => {
            match instruction.trim() {
                "NOP" => return Ok(vec![0x0]),
                "RLC" => return Ok(vec![0x7]),
                "RRC" => return Ok(vec![0x0f]),
                "RAL" => return Ok(vec![0x17]),
                "RAR" => return Ok(vec![0x1f]),
                "CMA" => return Ok(vec![0x2f]),
                "CMC" => return Ok(vec![0x3f]),
                "DAA" => return Ok(vec![0x27]),
                "HLT" => return Ok(vec![0x76]),
                "RNZ" => return Ok(vec![0xc0]),
                "STC" => return Ok(vec![0x37]),
                "RET" => return Ok(vec![0xc9]),
                "RNC" => return Ok(vec![0xd0]),
                "RZ" => return Ok(vec![0xc8]),
                "RC" => return Ok(vec![0xd8]),
                "RPE" => return Ok(vec![0xe8]),
                "RPO" => return Ok(vec![0xe0]),
                _ => return Err("Could not match instruction"),
            }
        }
    };
}

fn convert_mov_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
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

fn convert_stax_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    match args[0] {
        "B" => return Ok(vec![0x02]),
        "D" => return Ok(vec![0x12]),
        _ => return Err("wrong register!"),
    }
}

fn convert_inx_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    match args[0] {
        "B" => return Ok(vec![0x03]),
        "D" => return Ok(vec![0x13]),
        "H" => return Ok(vec![0x23]),
        "SP" => return Ok(vec![0x33]),
        _ => return Err("wrong register!"),
    }
}

fn convert_inr_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    let base_value = 0x04;
    match args[0] {
        "B" => return Ok(vec![base_value]),
        "C" => return Ok(vec![base_value + 1 * 8]),
        "D" => return Ok(vec![base_value + 2 * 8]),
        "E" => return Ok(vec![base_value + 3 * 8]),
        "H" => return Ok(vec![base_value + 4 * 8]),
        "L" => return Ok(vec![base_value + 5 * 8]),
        "M" => return Ok(vec![base_value + 6 * 8]),
        "A" => return Ok(vec![base_value + 7 * 8]),
        _ => return Err("wrong register!"),
    }
}

fn convert_dcr_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    let base_value = 0x05;
    match args[0] {
        "B" => return Ok(vec![base_value]),
        "C" => return Ok(vec![base_value + 1 * 8]),
        "D" => return Ok(vec![base_value + 2 * 8]),
        "E" => return Ok(vec![base_value + 3 * 8]),
        "H" => return Ok(vec![base_value + 4 * 8]),
        "L" => return Ok(vec![base_value + 5 * 8]),
        "M" => return Ok(vec![base_value + 6 * 8]),
        "A" => return Ok(vec![base_value + 7 * 8]),
        _ => return Err("wrong register!"),
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
    fn test_mov_operations() -> io::Result<()> {
        let input_codes = get_bytes_and_args_by_opcode("MOV").unwrap();

        for (bytes, arg_string) in input_codes {
            let args: Vec<&str> = arg_string.split(",").collect();
            assert_eq!(bytes, convert_mov_args(args).unwrap());
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

    #[test]
    fn test_opcodes_without_args() {
        let assembler = Assembler::new("RRC");
        assert_eq!(vec![0x0f], assembler.assemble().unwrap());

        let assembler = Assembler::new("RAL");
        assert_eq!(vec![0x17], assembler.assemble().unwrap());

        let assembler = Assembler::new("RAR");
        assert_eq!(vec![0x1f], assembler.assemble().unwrap());

        let assembler = Assembler::new("DAA");
        assert_eq!(vec![0x27], assembler.assemble().unwrap());

        let assembler = Assembler::new("CMA");
        assert_eq!(vec![0x2f], assembler.assemble().unwrap());

        let assembler = Assembler::new("STC");
        assert_eq!(vec![0x37], assembler.assemble().unwrap());

        let assembler = Assembler::new("CMC");
        assert_eq!(vec![0x3f], assembler.assemble().unwrap());

        let assembler = Assembler::new("HLT");
        assert_eq!(vec![0x76], assembler.assemble().unwrap());

        let assembler = Assembler::new("RNZ");
        assert_eq!(vec![0xc0], assembler.assemble().unwrap());

        let assembler = Assembler::new("RZ");
        assert_eq!(vec![0xc8], assembler.assemble().unwrap());

        let assembler = Assembler::new("RET");
        assert_eq!(vec![0xc9], assembler.assemble().unwrap());

        let assembler = Assembler::new("RNC");
        assert_eq!(vec![0xd0], assembler.assemble().unwrap());

        let assembler = Assembler::new("RC");
        assert_eq!(vec![0xd8], assembler.assemble().unwrap());

        let assembler = Assembler::new("RPO");
        assert_eq!(vec![0xe0], assembler.assemble().unwrap());

        let assembler = Assembler::new("RPE");
        assert_eq!(vec![0xe8], assembler.assemble().unwrap());
    }

    #[test]
    fn test_stax() {
        let inputs = get_bytes_and_args_by_opcode("STAX").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_stax_args(args).unwrap());
        }
    }

    #[test]
    fn test_stax_errors() {
        assert_eq!(Err("wrong register!"), convert_stax_args(vec!["L"]));
        assert_eq!(Err("wrong arg amount!"), convert_stax_args(vec!["L", "A"]));
        assert_eq!(Err("wrong arg amount!"), convert_stax_args(vec![]));
    }

    #[test]
    fn test_inx() {
        let inputs = get_bytes_and_args_by_opcode("INX").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_inx_args(args).unwrap());
        }
    }

    #[test]
    fn test_inx_errors() {
        assert_eq!(Err("wrong register!"), convert_inx_args(vec!["A"]));
        assert_eq!(Err("wrong arg amount!"), convert_inx_args(vec!["B", "D"]));
        assert_eq!(Err("wrong arg amount!"), convert_inx_args(vec![]));
    }

    #[test]
    fn test_inr() {
        let inputs = get_bytes_and_args_by_opcode("INR").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_inr_args(args).unwrap());
        }
    }

    #[test]
    fn test_inr_errors() {
        assert_eq!(Err("wrong register!"), convert_inr_args(vec!["Q"]));
        assert_eq!(Err("wrong arg amount!"), convert_inr_args(vec!["B", "D"]));
        assert_eq!(Err("wrong arg amount!"), convert_inr_args(vec![]));
    }

    #[test]
    fn test_dcr() {
        let inputs = get_bytes_and_args_by_opcode("DCR").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_dcr_args(args).unwrap());
        }
    }

    #[test]
    fn test_dcr_errors() {
        assert_eq!(Err("wrong register!"), convert_dcr_args(vec!["Q"]));
        assert_eq!(Err("wrong arg amount!"), convert_dcr_args(vec!["B", "D"]));
        assert_eq!(Err("wrong arg amount!"), convert_dcr_args(vec![]));
    }

    fn get_bytes_and_args_by_opcode(opcode: &str) -> io::Result<Vec<(Vec<u8>, String)>> {
        let f = File::open(OPCODE_TEST_DATA)?;
        let mut lines = io::BufReader::new(f).lines();
        let mut instructions = Vec::new();
        
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            if line.contains(opcode) {
                let components: Vec<&str> = line.split(":").collect();
                let bytes_str: Vec<&str> = components[0].split(",").collect();
                let args = String::from(components[1].split(" ").skip(1).next().unwrap());

                let mut bytes = Vec::new();
                for byte in bytes_str {
                    bytes.push(byte.parse::<u8>().unwrap());
                }
                instructions.push((bytes, args));
            }
        }
        Ok(instructions)
    }
}
