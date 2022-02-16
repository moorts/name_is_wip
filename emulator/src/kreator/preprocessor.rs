use super::assembler::{get_reserved_names, LABEL_DECL};
use super::parser::*;
use std::collections::HashMap;
use regex::Regex;

pub fn get_preprocessed_code(code: &Vec<String>) -> Result<Vec<String>, &'static str> {
    let decl_regex = Regex::new(LABEL_DECL).unwrap();
    if !has_correct_end(code) {
        return Err("A program must only contain one END statement and it has to be the last");
    }

    let mut equate_assignments: HashMap<String, u16> = HashMap::new();
    let mut set_assignments: HashMap<String, u16> = HashMap::new();
    let mut in_conditional = false;
    let mut condition = false;
    let mut preprocessed_code: Vec<String> = Vec::new();
    let mut pc = 0;

    let labels = get_labels(code)?;
    let code = replace_macros(code)?;

    for line in code {
        let mut owned_line = line.trim().to_string();

        // replace program counter references
        owned_line = owned_line.replace("$", &pc.to_string());

        // remove declaration of labels
        while let Some(_) = decl_regex.find(&owned_line) {
            owned_line = decl_regex.replace(&owned_line, "").to_string();
        }

        // replace labels with according values
        for (key, value) in &labels {
            owned_line = owned_line.replace(key, &value.to_string());
        }

        // determine if a variable is being declared by EQU
        if owned_line.contains("EQU") {
            let (name, expression) = owned_line.split_once(" EQU ").unwrap();
            if equate_assignments.contains_key(name) {
                return Err("Can't assign a variable more than once using EQU!");
            }
            equate_assignments.insert(name.to_string(), eval_str(expression.to_string()));
            continue;
        }

        // determine if a variable is being declared by SET
        if owned_line.contains("SET") {
            let (name, expression) = owned_line.split_once(" SET ").unwrap();
            set_assignments.insert(name.to_string(), eval_str(expression.to_string()));
            continue;
        }

        // replace values of variables declared by EQU
        for (key, value) in &equate_assignments {
            owned_line = owned_line.replace(&format!(" {}", key), &format!(" {}", value));
        }

        // replace values of variables declared by SET
        for (key, value) in &set_assignments {
            owned_line = owned_line.replace(&format!(" {}", key), &format!(" {}", value));
        }

        // check if conditional is exited (before check for entering since "IF" is contained in "ENDIF")
        if owned_line.contains("ENDIF") {
            if !in_conditional {
                return Err("Every ENDIF must have a corresponding IF");
            }
            condition = false;
            in_conditional = false;
            continue;
        }
        // check if conditional is being entered
        else if owned_line.contains("IF") {
            in_conditional = true;
            let condition_str = owned_line.split_once(" ").unwrap().1.to_string();
            condition = eval_str(condition_str) != 0;
            continue;
        }

        // check if conditional holds true
        if in_conditional {
            if !condition {
                continue;
            }
        }

        pc += 1;
        if !owned_line.is_empty() {
            preprocessed_code.push(owned_line.trim().to_string());
        } else {
            pc -= 1;
        }
    }

    if in_conditional {
        return Err("Every IF must be closed");
    }

    // remove "END" from code
    preprocessed_code.remove(preprocessed_code.len() - 1);
    Ok(preprocessed_code)
}

fn replace_macros(code: &Vec<String>) -> Result<Vec<String>, &'static str> {
    let (macro_instructions, macro_params) = get_macros(code)?;
    let mut macroless_code: Vec<String> = Vec::new();
    let mut in_macro_declaration = false;

    'outer: for line in code {
        let owned_line = line.trim().to_string();

        // check if macro is being declared
        if line.contains("MACRO") {
            in_macro_declaration = true;
            continue;
        }

        // check lines for ENDM statement
        if line.contains("ENDM") {
            in_macro_declaration = false;
            continue;
        }

        // remove lines of macro declaration
        if in_macro_declaration {
            continue;
        }

        for (macro_name, instructions) in &macro_instructions {
            if owned_line.contains(macro_name) {
                let input_string = owned_line.split_once(macro_name).unwrap().1.trim();
                let mut inputs: Vec<&str> = Vec::new();
                for input in input_string.split(",") {
                    inputs.push(input.trim());
                }
                let mut input_map: HashMap<String, String> = HashMap::new();
                for (index, parameter) in macro_params.get(macro_name).unwrap().iter().enumerate() {
                    let value = if index >= inputs.len() {
                        String::new()
                    } else {
                        inputs[index].to_string()
                    };
                    input_map.insert(parameter.to_string(), value);
                }
                for instruction in instructions {
                    let mut line = instruction.to_string();
                    for (variable, value) in &input_map {
                        let var_regex = Regex::new(&format!(r"[ ,]{}[ ,+\-*/,].", variable)).unwrap();
                        let end_regex = Regex::new(&format!("[ ,]{} ?$", variable)).unwrap();
                        while let Some(reg_match) = var_regex.find(&line) {
                            let first_match_symbol = line.get(reg_match.start()..reg_match.start() + 1).unwrap();
                            let last_match_symbol =  line.get(reg_match.end()..reg_match.end() + 1).unwrap();
                            let start = match first_match_symbol {
                                " " | "," => reg_match.start() + 1,
                                _ => reg_match.start()
                            };
                            let end = match last_match_symbol {
                                " " | "," | "+" | "-" | "*" | "/" => reg_match.end() - 2,
                                _ => reg_match.end() -1
                            };
                            line.replace_range(start..end - 1, &value);
                        }
                        if let Some(reg_match) = end_regex.find(&line) {
                            let first_symbol = line.get(reg_match.start()..reg_match.start() + 1).unwrap();
                            let start = match first_symbol {
                                " " | "," => reg_match.start() + 1,
                                _ => reg_match.start()
                            };
                            line.replace_range(start..reg_match.end(), &value);
                        }
                    }
                    macroless_code.push(line.trim().to_string());
                }
                continue 'outer;
            }
        }

        macroless_code.push(owned_line.trim().to_string());
    }
    Ok(macroless_code)
}

fn eval_str(str: String) -> u16 {
    let tokens = tokenize(str.to_string());
    to_expression_tree(tokens).evaluate() as u16
}

fn get_labels(code: &Vec<String>) -> Result<HashMap<String, u16>, &'static str> {
    let label_regex = Regex::new(LABEL_DECL).unwrap();
    let reserved_names = vec![
        "STC", "CMC", "INR", "DCR", "CMA", "DAA", "NOP", "MOV", "STAX", "LDAX", "ADD", "ADC",
        "SUB", "SBB", "ANA", "XRA", "ORA", "CMP", "RLC", "RRC", "RAL", "RAR", "PUSH", "POP", "DAD",
        "INX", "DCX", "XCHG", "XTHL", "SPHL", "LXI", "MVI", "ADI", "ACI", "SUI", "SBI", "ANI",
        "XRI", "ORI", "CPI", "STA", "LDA", "SHLD", "LHLD", "PCHL", "JMP", "JC", "JNC", "JZ", "JNZ",
        "JP", "JM", "JPE", "JPO", "CALL", "CC", "CNC", "CZ", "CNZ", "CP", "CM", "CPE", "CPO",
        "RET", "RC", "RNC", "RZ", "RNZ", "RM", "RP", "RPE", "RPO", "RST", "EI", "DI", "IN", "OUT",
        "HLT", "ORG", "EQU", "SET", "END", "IF", "ENDIF", "MACRO", "ENDM", "B", "C", "D", "H", "L",
        "A", "SP", "PSW"
    ];
    let mut temp_labels = Vec::new();
    let mut labels = HashMap::new();
    let mut mem_address = 0;

    for line in code {
        if label_regex.is_match(&line) {
            let split = line.split(":").collect::<Vec<&str>>();
            let label = split[0].trim_start();
            if reserved_names.contains(&label) {
                return Err("illegal label name");
            }
            temp_labels.push(label.to_string());
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
    let name_regex = Regex::new(r"^( *[a-zA-Z@?][a-zA-Z@?0-9]{0,4})").unwrap();

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
            if !name_regex.is_match(&macro_name)
                || get_reserved_names().iter().any(|&name| name == &macro_name)
            {
                return Err("Illegal macro name supplied!");
            }
            for parameter in split[1].split(",") {
                if !parameter.is_empty() {
                    current_parameters.push(parameter.trim().to_string());
                }
            }
            continue;
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
                return Err("Every ENDM must have a corresponding MACRO");
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
        if line.trim().contains("END") && !line.contains("ENDIF") && !line.contains("ENDM") {
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
    fn preprocessing_pc() {
        let code = vec!["MOV A,B", "JMP $", "END"];
        let ppc = get_preprocessed_code(&convert_input(code));
        assert_eq!(Ok(convert_input(vec!["MOV A,B", "JMP 1"])), ppc);

        let preprocessed_code = get_preprocessed_code(&convert_input(vec!["MOV $, $", "END"]));
        assert_eq!(Ok(vec!["MOV 0, 0".to_string()]), preprocessed_code);
    }

    #[test]
    fn remove_label_declarations() {
        let code = vec!["label:", "MOV A,B", "@LAB:", "test:", "MOV A,B", "END"];
        let ppc = get_preprocessed_code(&convert_input(code));

        assert_eq!(Ok(convert_input(vec!["MOV A,B", "MOV A,B"])), ppc);
    }

    #[test]
    fn illegal_label_declarations() {
        let label_wrapper = get_labels(&convert_input(vec!["A: MOV A,B"]));
        assert_eq!(Err("illegal label name"), label_wrapper);

        let label_wrapper = get_labels(&convert_input(vec!["LAB: MOV A,B", "LAB: RRC"]));
        assert_eq!(Err("label must not be assigned twice"), label_wrapper);
    }

    #[test]
    fn label_replacement() {
        let ppc = get_preprocessed_code(&convert_input(vec!["lab: lab", "END"]));
        assert_eq!(Ok(vec!["0".to_string()]), ppc);

        let ppc =
            get_preprocessed_code(&convert_input(vec!["MOV A, lab", "lab: RRC", "END"]));
        assert_eq!(Ok(convert_input(vec!["MOV A, 1", "RRC"])), ppc);
    }

    #[test]
    fn equate() {
        let ppc = get_preprocessed_code(&convert_input(vec!["PTO EQU 8", "OUT PTO", "END"]));
        assert_eq!(Ok(vec!["OUT 8".to_string()]), ppc);

        let ppc = get_preprocessed_code(&convert_input(vec!["test EQU 10H + 20", "JMP test", "END"]));
        assert_eq!(Ok(vec!["JMP 36".to_string()]), ppc);

        let ppc = get_preprocessed_code(&convert_input(vec!["test EQU 5", "test EQU 6", "END"]));
        assert_eq!(Err("Can't assign a variable more than once using EQU!"), ppc);
    }

    #[test]
    fn set() {
        let code = vec![
            "IMMED SET 5 ",
            "ADI IMMED",
            "IMMED SET 10H-6",
            "ADI IMMED",
            "END",
        ];
        let ppc = get_preprocessed_code(&convert_input(code));
        assert_eq!(Ok(convert_input(vec!["ADI 5", "ADI 10"])), ppc);
    }

    #[test]
    fn if_endif() {
        let code = vec![
            "COND SET 0ffH",
            "IF COND",
            "MOV A,C",
            "ENDIF",
            "COND SET 0",
            "IF COND ",
            "MOV A,C",
            "ENDIF",
            "XRA C",
            "END",
        ];
        let ppc = get_preprocessed_code(&convert_input(code));
        assert_eq!(Ok(convert_input(vec!["MOV A,C", "XRA C"])), ppc);

        let ppc = get_preprocessed_code(&convert_input(vec!["IF 1", "END"]));
        assert_eq!(Err("Every IF must be closed"), ppc);

        let ppc = get_preprocessed_code(&convert_input(vec!["ENDIF", "END"]));
        assert_eq!(Err("Every ENDIF must have a corresponding IF"), ppc);
    }

    #[test]
    fn macro_replacement() {
        let code = &convert_input(vec!["SHRT MACRO", "RRC", "ANI 7FH", "ENDM", "SHRT", "END"]);
        let ppc = replace_macros(code);
        assert_eq!(Ok(convert_input(vec!["RRC", "ANI 7FH", "END"])), ppc);

        let code = &convert_input(vec!["SHRT MACRO", "RRC", "ANI 7FH", "ENDM", "END"]);
        let ppc = replace_macros(code);
        assert_eq!(Ok(convert_input(vec!["END"])), ppc);

        let code = &convert_input(vec![
            "MAC1 MACRO P1, P2,COMMENT",
            "XRA P2",
            "DCR P1 COMMENT",
            "ENDM",
            "MAC1 C, D",
            "END",
        ]);
        let ppc = replace_macros(code);
        assert_eq!(Ok(convert_input(vec!["XRA D", "DCR C", "END"])), ppc);

        let code = &convert_input(vec!["MA MACRO Foo, FooBar", "MOV Foo, FooBar", "ENDM", "MA A, B"]);
        let ppc = replace_macros(code);
        assert_eq!(Ok(convert_input(vec!["MOV A, B"])), ppc);
    }

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

        let code = convert_input(vec![
            "MAC1 MACRO P1, P2, COMMENT",
            "XRA P2",
            "DCR P1 COMMENT",
            "ENDM",
            "MAC1 C, D",
        ]);
        let mut params = HashMap::new();
        params.insert("MAC1".to_string(), convert_input(vec!["P1", "P2", "COMMENT"]));
        assert_eq!(params, get_macros(&code).unwrap().1);

        let code = convert_input(vec!["THE MACRO"]);
        assert_eq!(Err("Every MACRO has to be followed by an ENDM"), get_macros(&code));

        let code = convert_input(vec!["ENDM"]);
        assert_eq!(Err("Every ENDM must have a corresponding MACRO"), get_macros(&code));

        let code = convert_input(vec!["MACRO", "ENDM", "END"]);
        assert_eq!(Err("Cannot define macro without name"), get_macros(&code));

        let code = convert_input(vec!["ABC MACRO", "A MACRO", "ENDM"]);
        assert_eq!(Err("Cannot define macro within macro"), get_macros(&code));

        let code = convert_input(vec!["A MACRO", "ENDM"]);
        assert_eq!(Err("Illegal macro name supplied!"), get_macros(&code));
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

    #[test]
    fn complete_code() {
        let code = convert_input(vec![
            "VAR1 EQU 123",
            "GO: JMP $ +6",
            "ADD C",
            "",
            "",
            "IF 0+0*00O",
            "MOV A,B",
            "ENDIF",
            "POP B",
            "macr0 MACRO par",
            "NOM SET 21",
            "RZ",
            "ENDM",
            "",
            "macr0 input",
            "IF NOM",
            "EI",
            "ENDIF",
            "END",
            "",
        ]);

        let result = convert_input(vec!["JMP 0 +6", "ADD C", "POP B", "RZ", "EI"]);

        assert_eq!(Ok(result), get_preprocessed_code(&code));
    }

    fn convert_input(lines: Vec<&str>) -> Vec<String> {
        let mut string_vector: Vec<String> = Vec::new();
        for line in lines {
            string_vector.push(line.to_string())
        }
        string_vector
    }
}
