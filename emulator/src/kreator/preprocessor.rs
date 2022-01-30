use std::collections::HashMap;
use super::assembler::LABEL_DECL;

use regex::Regex;


pub fn get_labels(code: &Vec<String>) -> Result<HashMap<String, u16>, &'static str> {
    let label_regex = Regex::new(LABEL_DECL).unwrap();
        let reserved_names = vec![
            "STC", "CMC", "INR", "DCR", "CMA", "DAA", "NOP", "MOV", "STAX", "LDAX", "ADD", "ADC",
            "SUB", "SBB", "ANA", "XRA", "ORA", "CMP", "RLC", "RRC", "RAL", "RAR", "PUSH", "POP",
            "DAD", "INX", "DCX", "XCHG", "XTHL", "SPHL", "LXI", "MVI", "ADI", "ACI", "SUI", "SBI",
            "ANI", "XRI", "ORI", "CPI", "STA", "LDA", "SHLD", "LHLD", "PCHL", "JMP", "JC", "JNC",
            "JZ", "JNZ", "JP", "JM", "JPE", "JPO", "CALL", "CC", "CNC", "CZ", "CNZ", "CP", "CM",
            "CPE", "CPO", "RET", "RC", "RNC", "RZ", "RNZ", "RM", "RP", "RPE", "RPO", "RST", "EI",
            "DI", "IN", "OUT", "HLT", "ORG", "EQU", "SET", "END", "IF", "ENDIF", "MACRO", "ENDM",
            "B", "C", "D", "H", "L", "A", "SP", "PSW", 
        ];
        let mut temp_labels = Vec::new();
        let mut labels = HashMap::new();
        let mut mem_address = 0;

        for line in code {
            if label_regex.is_match(&line) {
                let split = line.split(":").collect::<Vec<&str>>();
                let label = split[0].trim_start().to_string();
                if reserved_names.iter().any(|&name| name == label) {
                    return Err("illegal label name");
                }
                temp_labels.push(label);
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

fn get_macros(code: &Vec<String>) -> Result<(HashMap<String, Vec<String>>, HashMap<String, Vec<String>>), &'static str> {
    let mut macros: HashMap<String, Vec<String>> = HashMap::new();
    let mut parameters: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_macro = false;
    let mut macro_name = String::new();

    let mut current_macro: Vec<String> = Vec::new();
    let mut current_parameters: Vec<String> = Vec::new();

    for line in code {
        let line = line.trim();

        if line.contains("MACRO") {
            if in_macro {
                return Err("Cannot define macro within macro");
            }
            in_macro = true;
            let split: Vec<&str> = line.split("MACRO").collect();
            macro_name = split[0].trim().to_string();
            if macro_name.is_empty() {
                return Err("Cannot define macro without name");
            }
            for parameter in split[1].split(",") {
                if !parameter.is_empty() {
                    current_parameters.push(parameter.trim().to_string());
                }
            }
            continue
        }
        if line.contains("ENDM") {
            if line != "ENDM" {
                return Err("ENDM must stand alone");
            }
            if in_macro {
                macros.insert(macro_name.to_string(), current_macro.to_owned());
                parameters.insert(macro_name.to_string(), current_parameters.to_owned());
                current_macro.clear();
                current_parameters.clear();
                macro_name.clear();
                in_macro = false;
            } else {
                return Err("Every ENDM must have a corresponding MACRO")
            }
        }
        if in_macro {
            current_macro.push(line.to_string());
        }
    }
    if in_macro {
        return Err("Every MACRO has to be followed by an ENDM");
    }
    Ok((macros, parameters))
}

fn has_correct_end(code: &Vec<String>) -> bool {
    let mut has_end = false;

    for line in code {
        if line.is_empty() {
            continue;
        }
        if line.trim().contains("END") {
            if has_end {
                return false;
            }
            has_end = true;
            continue;
        }
        if has_end {
            return false;
        }
    }
    return has_end;
}

mod tests {
    use super::*;

    #[test]
    fn valid_labels() {
        let code = convert_input(vec!["label:", "MOV A,B", " @LAB:", "test:", "MOV A,B"]);
        let mut labels = HashMap::new();
        labels.insert(String::from("test"), 1);
        labels.insert(String::from("@LAB"), 1);
        labels.insert(String::from("label"), 0);

        assert_eq!(Ok(labels), get_labels(&code));
    }

    #[test]
    fn duplicate_labels() {
        let labels = get_labels(&convert_input(vec!["label:", "label:", "MOV A,B"]));
        assert_eq!(Err("label must not be assigned twice!"), labels);
    }

    #[test]
    fn empty_label() {
        let labels = get_labels(&convert_input(vec!["label:"]));
        assert_eq!(Err("labels must not point to an empty address!"), labels);
    }

    #[test]
    fn illegal_label() {
        let labels = get_labels(&vec!["IF: RRC".to_string()]);
        assert_eq!(Err("illegal label name"), labels);
    }

    #[test]
    fn macro_definitions() {
        let code = convert_input(vec!["SHRT MACRO", "RRC", "ANI 7FH", "ENDM", "SHRT"]);
        let mut instructions = HashMap::new();
        instructions.insert("SHRT".to_string(), convert_input(vec!["RRC", "ANI 7FH"]));
        assert_eq!(instructions, get_macros(&code).unwrap().0);

        let code = convert_input(vec!["MAC1 MACRO P1, P2, COMMENT", "XRA P2", "DCR P1 COMMENT", "ENDM", "MAC1 C, D"]);
        let mut params = HashMap::new();
        params.insert("MAC1".to_string(), convert_input(vec!["P1", "P2", "COMMENT"]));
        assert_eq!(params, get_macros(&code).unwrap().1);

        let code = convert_input(vec!["A MACRO"]);
        assert_eq!(Err("Every MACRO has to be followed by an ENDM"), get_macros(&code));

        let code = convert_input(vec!["ENDM"]);
        assert_eq!(Err("Every ENDM must have a corresponding MACRO"), get_macros(&code));

        let code = convert_input(vec!["MACRO", "ENDM", "END"]);
        assert_eq!(Err("Cannot define macro without name"), get_macros(&code));

        let code = convert_input(vec!["ABC MACRO", "A MACRO", "ENDM"]);
        assert_eq!(Err("Cannot define macro within macro"), get_macros(&code));
    }

    #[test]
    fn program_has_end() {
        let code = convert_input(vec!["END"]);
        assert_eq!(true, has_correct_end(&code));

        let code = convert_input(vec!["END", "END"]);
        assert_eq!(false, has_correct_end(&code));

        let code = convert_input(vec!["RRC"]);
        assert_eq!(false, has_correct_end(&code));

        let code = convert_input(vec!["END", "RRC"]);
        assert_eq!(false, has_correct_end(&code));
    }


    fn convert_input(lines: Vec<&str>) -> Vec<String> {
        let mut string_vector: Vec<String> = Vec::new();
        for line in lines {
            string_vector.push(line.to_string())
        }
        string_vector
    }
}
