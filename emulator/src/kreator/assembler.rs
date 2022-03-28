use super::parser::eval;
use super::preprocessor::{get_preprocessed_code, get_line_map};
use core::fmt;
use regex::Regex;
use std::{collections::HashMap, hash::Hash};

pub const LABEL_DECL: &str = r"^( *[a-zA-Z@?][a-zA-Z@?0-9]{0,4}:)";

pub fn get_reserved_names() -> Vec<&'static str> {
    vec![
        "STC", "CMC", "INR", "DCR", "CMA", "DAA", "NOP", "MOV", "STAX", "LDAX", "ADD", "ADC",
        "SUB", "SBB", "ANA", "XRA", "ORA", "CMP", "RLC", "RRC", "RAL", "RAR", "PUSH", "POP",
        "DAD", "INX", "DCX", "XCHG", "XTHL", "SPHL", "LXI", "MVI", "ADI", "ACI", "SUI", "SBI",
        "ANI", "XRI", "ORI", "CPI", "STA", "LDA", "SHLD", "LHLD", "PCHL", "JMP", "JC", "JNC",
        "JZ", "JNZ", "JP", "JM", "JPE", "JPO", "CALL", "CC", "CNC", "CZ", "CNZ", "CP", "CM",
        "CPE", "CPO", "RET", "RC", "RNC", "RZ", "RNZ", "RM", "RP", "RPE", "RPO", "RST", "EI",
        "DI", "IN", "OUT", "HLT", "ORG", "EQU", "SET", "END", "IF", "ENDIF", "MACRO", "ENDM",
        "B", "C", "D", "H", "L", "A", "SP", "PSW",
    ]
}

pub struct Assembler {
    code: Vec<String>,
}

impl fmt::Display for Assembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.join("\n"))
    }
}

impl Assembler {
    pub fn new(input_code: &str) -> Self {
        let mut lines:Vec<String> = Vec::new();

        for line in input_code.split("\n") {
            lines.push(line.trim().to_string());
        }

        Self { code: lines }
    }

    pub fn assemble(&self) -> Result<Vec<u8>, &'static str> {
        let label_regex = Regex::new(LABEL_DECL).unwrap();
        let preprocessed_code = get_preprocessed_code(&self.code)?;

        let mut machine_code = Vec::new();

        for line in preprocessed_code {
            let line = label_regex.replace(&line, "").trim().to_string();

            if !line.is_empty() && !line.contains("ORG ") {
                machine_code.extend(to_machine_code(line)?);
            }
        }
        Ok(machine_code)
    }

    pub fn get_line_map(&self) -> Result<HashMap<u16, usize>, &'static str> {
        get_line_map(&self.code)
    }

    pub fn get_origins(&self) -> Vec<(u16, u16)> {
        let label_regex = Regex::new(LABEL_DECL).unwrap();
        let mut origins: Vec<(u16, u16)> = Vec::new();
        let mut executed_bytes = 0;

        for line in get_preprocessed_code(&self.code).unwrap() {
            if line.contains("ORG") {
                let split = line.split_once(" ").unwrap();
                origins.push((executed_bytes, evaluate_str(split.1)));
            } else {
                let line = label_regex.replace(&line, "").to_string();
                executed_bytes = executed_bytes + to_machine_code(line).unwrap().len() as u16;
            }
        }
        origins
    }
}

fn to_machine_code(instruction: String) -> Result<Vec<u8>, &'static str> {
    let label_regex = Regex::new(LABEL_DECL).unwrap();
    let instruction = label_regex.replace(&instruction, "").to_string();
    let mut args: Vec<&str> = Vec::new();

    match instruction.trim_start().split_once(" ") {
        Some((opcode, suffix)) => {
            let dirty_args: Vec<&str> = suffix.split(",").collect();
            for arg in dirty_args {
                args.push(arg.trim());
            }
            match opcode {
                "MOV" => return convert_mov_args(args),
                "STAX" => return convert_stax_args(args),
                "INX" => return convert_inx_args(args),
                "INR" => return convert_opcodes_using_all_registers(args, 0x04, true),
                "DCR" => return convert_opcodes_using_all_registers(args, 0x05, true),
                "ADD" => return convert_opcodes_using_all_registers(args, 0x80, false),
                "ADC" => return convert_opcodes_using_all_registers(args, 0x88, false),
                "SUB" => return convert_opcodes_using_all_registers(args, 0x90, false),
                "SBB" => return convert_opcodes_using_all_registers(args, 0x98, false),
                "ANA" => return convert_opcodes_using_all_registers(args, 0xa0, false),
                "XRA" => return convert_opcodes_using_all_registers(args, 0xa8, false),
                "ORA" => return convert_opcodes_using_all_registers(args, 0xb0, false),
                "CMP" => return convert_opcodes_using_all_registers(args, 0xb8, false),
                "LXI" => return convert_lxi_args(args),
                "MVI" => return convert_mvi_args(args),
                "DAD" => return convert_dad_args(args),
                "DCX" => return convert_dcx_args(args),
                "POP" => return convert_pop_args(args),
                "RST" => return convert_rst_args(args),
                "PUSH" => return convert_push_args(args),
                "LDAX" => match args[0] {
                    "B" => return Ok(vec![0x0a]),
                    "D" => return Ok(vec![0x1a]),
                    _ => return Err("wrong register!"),
                },
                "SHLD" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0x22, adr as u8, (adr >> 8) as u8]);
                }
                "LHLD" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0x2a, adr as u8, (adr >> 8) as u8]);
                }
                "STA" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0x32, adr as u8, (adr >> 8) as u8]);
                }
                "LDA" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0x3a, adr as u8, (adr >> 8) as u8]);
                }
                "JMP" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xc3, adr as u8, (adr >> 8) as u8]);
                }
                "JNZ" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xc2, adr as u8, (adr >> 8) as u8]);
                }
                "CNZ" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xc4, adr as u8, (adr >> 8) as u8]);
                }
                "ADI" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xc6, adr as u8]);
                }
                "JZ" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xca, adr as u8, (adr >> 8) as u8]);
                }
                "CZ" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xcc, adr as u8, (adr >> 8) as u8]);
                }
                "CALL" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xcd, adr as u8, (adr >> 8) as u8]);
                }
                "ACI" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xce, adr as u8]);
                }
                "JNC" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xd2, adr as u8, (adr >> 8) as u8]);
                }
                "OUT" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xd3, adr as u8]);
                }
                "CNC" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xd4, adr as u8, (adr >> 8) as u8]);
                }
                "SUI" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xd6, adr as u8]);
                }
                "JC" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xda, adr as u8, (adr >> 8) as u8]);
                }
                "IN" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xdb, adr as u8]);
                }
                "CC" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xdc, adr as u8, (adr >> 8) as u8]);
                }
                "SBI" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xde, adr as u8]);
                }
                "JPO" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xe2, adr as u8, (adr >> 8) as u8]);
                }
                "CPO" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xe4, adr as u8, (adr >> 8) as u8]);
                }
                "ANI" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xe6, adr as u8]);
                }
                "JPE" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xea, adr as u8, (adr >> 8) as u8]);
                }
                "CPE" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xec, adr as u8, (adr >> 8) as u8]);
                }
                "XRI" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xee, adr as u8]);
                }
                "JP" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xf2, adr as u8, (adr >> 8) as u8]);
                }
                "CP" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xf4, adr as u8, (adr >> 8) as u8]);
                }
                "ORI" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xf6, adr as u8]);
                }
                "JM" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xfa, adr as u8, (adr >> 8) as u8]);
                }
                "CM" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xfc, adr as u8, (adr >> 8) as u8]);
                }
                "CPI" => {
                    let adr = evaluate_str(args[0]);
                    return Ok(vec![0xfe, adr as u8]);
                }
                _ => return Err("Could not match instruction"),
            }
        }
        None => match instruction.trim() {
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
            "RPE" => return Ok(vec![0xe8]),
            "RPO" => return Ok(vec![0xe0]),
            "EI" => return Ok(vec![0xfb]),
            "RM" => return Ok(vec![0xf8]),
            "RZ" => return Ok(vec![0xc8]),
            "RC" => return Ok(vec![0xd8]),
            "DI" => return Ok(vec![0xf3]),
            "RP" => return Ok(vec![0xf0]),
            "SPHL" => return Ok(vec![0xf9]),
            "XCHG" => return Ok(vec![0xeb]),
            "PCHL" => return Ok(vec![0xe9]),
            "XTHL" => return Ok(vec![0xe3]),
            _ => return Err("Could not match instruction"),
        },
    };
}

fn evaluate_str(str: &str) -> u16 {
    eval(str) as u16
}

fn convert_mov_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    let base_value = 0x40;
    let registers = "BCDEHLMA";

    match args.len() {
        0 | 1 => return Err("Missing argument(s) for MOV instruction"),
        2 => match registers.find(args[0]) {
            Some(index) => match registers.find(args[1]) {
                Some(second_index) => {
                    if index == 6 && second_index == 6 {
                        return Err("Invalid arguments for MOV instruction (Can't move M into M)");
                    }
                    let instruction_value = base_value + (index as u8 * 8) + second_index as u8;
                    return Ok(vec![instruction_value]);
                }
                None => return Err("Invalid second argument for MOV instruction"),
            },
            None => return Err("Invalid first argument for MOV instruction"),
        },
        _ => return Err("MOV only takes 2 arguments!"),
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

fn convert_opcodes_using_all_registers(
    args: Vec<&str>,
    base_value: u8,
    use_every_eigth_opc: bool,
) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    let growth = if use_every_eigth_opc { 8 } else { 1 };
    match args[0] {
        "B" => return Ok(vec![base_value]),
        "C" => return Ok(vec![base_value + (1 * growth)]),
        "D" => return Ok(vec![base_value + (2 * growth)]),
        "E" => return Ok(vec![base_value + (3 * growth)]),
        "H" => return Ok(vec![base_value + (4 * growth)]),
        "L" => return Ok(vec![base_value + (5 * growth)]),
        "M" => return Ok(vec![base_value + (6 * growth)]),
        "A" => return Ok(vec![base_value + (7 * growth)]),
        _ => return Err("wrong register!"),
    }
}

fn convert_lxi_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 2 {
        return Err("wrong arg amount!");
    }
    let imm_val = evaluate_str(args[1]);
    match args[0] {
        "B" => return Ok(vec![0x01, imm_val as u8, (imm_val >> 8) as u8]),
        "D" => return Ok(vec![0x11, imm_val as u8, (imm_val >> 8) as u8]),
        "H" => return Ok(vec![0x21, imm_val as u8, (imm_val >> 8) as u8]),
        "SP" => return Ok(vec![0x31, imm_val as u8, (imm_val >> 8) as u8]),
        _ => return Err("wrong register!"),
    }
}

fn convert_mvi_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 2 {
        return Err("wrong arg amount!");
    }
    let immediate_value = eval(args[1]) as u8;
    match args[0] {
        "B" => return Ok(vec![0x06, immediate_value]),
        "C" => return Ok(vec![0x0e, immediate_value]),
        "D" => return Ok(vec![0x16, immediate_value]),
        "E" => return Ok(vec![0x1e, immediate_value]),
        "H" => return Ok(vec![0x26, immediate_value]),
        "L" => return Ok(vec![0x2e, immediate_value]),
        "M" => return Ok(vec![0x36, immediate_value]),
        "A" => return Ok(vec![0x3e, immediate_value]),
        _ => return Err("wrong register!"),
    }
}

fn convert_dad_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    match args[0] {
        "B" => return Ok(vec![0x09]),
        "D" => return Ok(vec![0x19]),
        "H" => return Ok(vec![0x29]),
        "SP" => return Ok(vec![0x39]),
        _ => return Err("wrong register!"),
    }
}

fn convert_dcx_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    match args[0] {
        "B" => return Ok(vec![0x0b]),
        "D" => return Ok(vec![0x1b]),
        "H" => return Ok(vec![0x2b]),
        "SP" => return Ok(vec![0x3b]),
        _ => return Err("wrong register!"),
    }
}

fn convert_pop_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    match args[0] {
        "B" => return Ok(vec![0xc1]),
        "D" => return Ok(vec![0xd1]),
        "H" => return Ok(vec![0xe1]),
        "PSW" => return Ok(vec![0xf1]),
        _ => return Err("wrong register!"),
    }
}

fn convert_push_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    match args[0] {
        "B" => return Ok(vec![0xc5]),
        "D" => return Ok(vec![0xd5]),
        "H" => return Ok(vec![0xe5]),
        "PSW" => return Ok(vec![0xf5]),
        _ => return Err("wrong register!"),
    }
}

fn convert_rst_args(args: Vec<&str>) -> Result<Vec<u8>, &'static str> {
    if args.len() != 1 {
        return Err("wrong arg amount!");
    }
    let value = args[0].parse::<u8>().unwrap();
    if value <= 7 {
        return Ok(vec![0xc7 + value * 8]);
    }
    Err("wrong register!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, BufRead};

    const OPCODE_TEST_DATA: &str = "./test_data/test_input";

    #[test]
    fn display_with_code() {
        let code_file = "MOV A B \n JMP label \nlabel: INC ACC   ";
        let assembler = Assembler::new(code_file);

        let expected_text = "MOV A B\nJMP label\nlabel: INC ACC";
        assert_eq!(expected_text, format!("{}", assembler));
    }

    #[test]
    fn display_windows_newline() {
        let code_file = "MOV A B \r\n JMP label \r\nlabel: INC ACC  ";
        let assembler = Assembler::new(code_file);

        let expected_text = "MOV A B\nJMP label\nlabel: INC ACC";
        assert_eq!(expected_text, format!("{}", assembler));
    }

    #[test]
    fn display_without_code() {
        let assembler = Assembler::new("");

        assert_eq!("", format!("{}", assembler));
    }

    #[test]
    fn display_with_comments() {
        let code_file = " \n;comment\nMOV A B ;comment\n;";
        let assembler = Assembler::new(code_file);

        let expected_text = "\n;comment\nMOV A B ;comment\n;";
        assert_eq!(expected_text, format!("{}", assembler));
    }

    #[test]
    fn empty_code_file() {
        let assembler = Assembler::new("END");

        assert_eq!(0, assembler.assemble().unwrap().len());
    }

    #[test]
    fn mov_operations() {
        let input_codes = get_bytes_and_args_by_opcode("MOV").unwrap();

        for (bytes, arg_string) in input_codes {
            let args: Vec<&str> = arg_string.split(",").collect();
            assert_eq!(bytes, convert_mov_args(args).unwrap());
        }
    }

    #[test]
    fn mov_errors() {
        assert_eq!(Err("Missing argument(s) for MOV instruction"), to_machine_code("MOV A".to_string()));
        assert_eq!(Err("Invalid second argument for MOV instruction"), to_machine_code("MOV B,Q".to_string()));
        assert_eq!(Err("Invalid arguments for MOV instruction (Can't move M into M)"), to_machine_code("MOV M,M".to_string()));
        assert_eq!(Err("MOV only takes 2 arguments!"), to_machine_code("MOV A,B,C".to_string()));
    }

    #[test]
    fn nop_operation() {
        assert_eq!(Ok(vec![0x0]), to_machine_code("NOP".to_string()));
        assert_eq!(Err("Could not match instruction"), to_machine_code("NOP A".to_string()));
    }

    #[test]
    fn invalid_instructions() {
        assert_eq!(Err("Could not match instruction"), to_machine_code("TEST".to_string()));
    }

    #[test]
    fn opcodes_without_args() {
        let mut opcodes = HashMap::new();
        opcodes.insert("RRC", 0x0f);
        opcodes.insert("RAL", 0x17);
        opcodes.insert("RAR", 0x1f);
        opcodes.insert("DAA", 0x27);
        opcodes.insert("CMA", 0x2f);
        opcodes.insert("CMC", 0x3f);
        opcodes.insert("HLT", 0x76);
        opcodes.insert("RNZ", 0xc0);
        opcodes.insert("RZ", 0xc8);
        opcodes.insert("RET", 0xc9);
        opcodes.insert("RNC", 0xd0);
        opcodes.insert("RC", 0xd8);
        opcodes.insert("RPO", 0xe0);
        opcodes.insert("RPE", 0xe8);
        opcodes.insert("EI", 0xfb);
        opcodes.insert("RM", 0xf8);
        opcodes.insert("SPHL", 0xf9);
        opcodes.insert("DI", 0xf3);
        opcodes.insert("RP", 0xf0);
        opcodes.insert("XCHG", 0xeb);
        opcodes.insert("PCHL", 0xe9);
        opcodes.insert("XTHL", 0xe3);

        for (instruction, opc) in opcodes {
            assert_eq!(Ok(vec![opc]), to_machine_code(instruction.to_string()));
        }
    }

    #[test]
    fn convert_stax() {
        let inputs = get_bytes_and_args_by_opcode("STAX").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_stax_args(args).unwrap());
        }
    }

    #[test]
    fn stax_errors() {
        assert_eq!(Err("wrong register!"), convert_stax_args(vec!["L"]));
        assert_eq!(Err("wrong arg amount!"), convert_stax_args(vec!["L", "A"]));
        assert_eq!(Err("wrong arg amount!"), convert_stax_args(vec![]));
    }

    #[test]
    fn inx() {
        let inputs = get_bytes_and_args_by_opcode("INX").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_inx_args(args).unwrap());
        }
    }

    #[test]
    fn inx_errors() {
        assert_eq!(Err("wrong register!"), convert_inx_args(vec!["A"]));
        assert_eq!(Err("wrong arg amount!"), convert_inx_args(vec!["B", "D"]));
        assert_eq!(Err("wrong arg amount!"), convert_inx_args(vec![]));
    }

    #[test]
    fn opcodes_using_registersteps_of_8() {
        let inputs = get_bytes_and_args_by_opcode("INR").unwrap();
        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(
                input.0,
                convert_opcodes_using_all_registers(args, 4, true).unwrap()
            );
        }

        let inputs = get_bytes_and_args_by_opcode("DCR").unwrap();
        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(
                input.0,
                convert_opcodes_using_all_registers(args, 5, true).unwrap()
            );
        }
    }

    #[test]
    fn opcodes_using_registers() {
        let add_value = 0x80;
        let opcodes = vec!["ADD", "ADC", "SUB", "SBB", "ANA", "XRA", "ORA", "CMP"];

        for (index, &opcode) in opcodes.iter().enumerate() {
            let inputs = get_bytes_and_args_by_opcode(opcode).unwrap();
            for input in inputs {
                let args: Vec<&str> = input.1.split(",").collect();
                assert_eq!(
                    input.0,
                    convert_opcodes_using_all_registers(args, add_value + 8 * index as u8, false)
                        .unwrap()
                );
            }
        }
    }

    #[test]
    fn opcodes_using_registers_errors() {
        assert_eq!(
            Err("wrong arg amount!"),
            convert_opcodes_using_all_registers(vec!["B", "D"], 1, false)
        );
        assert_eq!(
            Err("wrong arg amount!"),
            convert_opcodes_using_all_registers(vec![], 1, false)
        );
    }

    #[test]
    fn convert_lxi() {
        let inputs = get_bytes_and_args_by_opcode("LXI").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_lxi_args(args).unwrap());
        }
    }

    #[test]
    fn convert_mvi() {
        let inputs = get_bytes_and_args_by_opcode("MVI").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_mvi_args(args).unwrap());
        }
    }

    #[test]
    fn convert_dad() {
        let inputs = get_bytes_and_args_by_opcode("DAD").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_dad_args(args).unwrap());
        }
    }

    #[test]
    fn convert_dcx() {
        let inputs = get_bytes_and_args_by_opcode("DCX").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_dcx_args(args).unwrap());
        }
    }

    #[test]
    fn convert_pop() {
        let inputs = get_bytes_and_args_by_opcode("POP").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_pop_args(args).unwrap());
        }
    }

    #[test]
    fn convert_push() {
        let inputs = get_bytes_and_args_by_opcode("PUSH").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_push_args(args).unwrap());
        }
    }

    #[test]
    fn convert_rst() {
        let inputs = get_bytes_and_args_by_opcode("RST").unwrap();

        for input in inputs {
            let args: Vec<&str> = input.1.split(",").collect();
            assert_eq!(input.0, convert_rst_args(args).unwrap());
        }
    }

    #[test]
    fn opcodes_in_assemble() {
        let opcodes = vec![
            "CZ", "JZ", "ADI", "CNZ", "JNZ", "JMP", "LDA", "STA", "LHLD", "SHLD", "LDAX", "CALL",
            "ACI", "JNC", "OUT", "CNC", "SUI", "JC", "IN", "CC", "SBI", "JPO", "CPO", "ANI", "JPE",
            "CPE", "XRI", "JP", "CP", "ORI", "JM", "CM", "CPI",
        ];

        for opc in opcodes {
            let inputs = get_bytes_and_args_by_opcode(opc).unwrap();

            for input in inputs {
                assert_eq!(Ok(input.0), to_machine_code(format!("{} {}", opc, &input.1)));
            }
        }
    }

    #[test]
    fn all_opcodes() {
        let f = File::open(OPCODE_TEST_DATA).unwrap();
        let mut lines = io::BufReader::new(f).lines();

        while let Some(line) = lines.next() {
            let component_binding = line.unwrap();
            if component_binding.contains("-") {
                continue;
            }
            let components: Vec<&str> = component_binding.split(":").collect();
            let mut bytes = Vec::new();
            for byte in components[0].split(",") {
                bytes.push(byte.parse::<u8>().unwrap());
            }
            let operation = components[1];
            let assembler = Assembler::new(format!("{}\nEND", operation).as_str());
            assert_eq!(bytes, assembler.assemble().unwrap());
        }
    }

    #[test]
    fn org_first_address() {
        let assembler = Assembler::new("RNC \n ORG 20H\nEND");
        assert_eq!(vec![(1, 32)], assembler.get_origins());

        let assembler = Assembler::new("RNC\nEND");
        assert_eq!(Vec::<(u16, u16)>::new(), assembler.get_origins());

        let assembler = Assembler::new("ORG 5 + 1 \nRNC\nEND");
        assert_eq!(vec![(0, 6)], assembler.get_origins());
    }

    #[test]
    fn multiple_orgs() {
        let assembler = Assembler::new(
            "ORG 1000H \n MOV A,C \n ADI 2\n JMP NEXT \n HERE:ORG 1050H \n NEXT: XRA A\nEND",
        );
        let jumps: Vec<(u16, u16)> = vec![(0, 0x1000), (6, 0x1050)];

        assert_eq!(jumps, assembler.get_origins());
    }

    #[test]
    fn full_program() {
        let code = "VAR1 EQU 123\n 
        GO: JMP $ +6\n
        ADD C\n
        \n
        \n
        IF 0+0*00O\n
        MOV A,B\n
        ENDIF\n
        POP B\n
        macr0 MACRO par\n
        NOM SET 21\n
        RZ\n
        ENDM\n
        \n
        macr0 input\n
        IF 20\n
        EI\n
        ENDIF\n
        ORG 100H\n
        END\n";

        let result = vec![0xc3, 0x6, 0x0, 0x81, 0xC1, 0xC8, 0xFB];
        
        assert_eq!(Ok(result), Assembler::new(code).assemble());
    }

    fn get_bytes_and_args_by_opcode(opcode: &str) -> io::Result<Vec<(Vec<u8>, String)>> {
        let f = File::open(OPCODE_TEST_DATA)?;
        let mut lines = io::BufReader::new(f).lines();
        let mut instructions = Vec::new();

        while let Some(line) = lines.next() {
            let line = line.unwrap();
            if line.contains(&format!("{} ", opcode)) {
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
