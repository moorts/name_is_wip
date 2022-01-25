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

mod tests {
    use super::*;

    #[test]
    fn valid_labels() {
        let code = vec!["label:".to_string(), "MOV A,B".to_string(), " @LAB:".to_string(), "test:".to_string(), "MOV A,B".to_string()];
        let mut labels = HashMap::new();
        labels.insert(String::from("test"), 1);
        labels.insert(String::from("@LAB"), 1);
        labels.insert(String::from("label"), 0);

        assert_eq!(Ok(labels), get_labels(&code));
    }

    #[test]
    fn duplicate_labels() {
        let labels = get_labels(&vec!["label:".to_string(), "label:".to_string(), "MOV A,B".to_string()]);
        assert_eq!(Err("label must not be assigned twice!"), labels);
    }

    #[test]
    fn empty_label() {
        let labels = get_labels(&vec!["label:".to_string()]);
        assert_eq!(Err("labels must not point to an empty address!"), labels);
    }

    #[test]
    fn illegal_label() {
        let labels = get_labels(&vec!["IF: RRC".to_string()]);
        assert_eq!(Err("illegal label name"), labels);
    }
}
