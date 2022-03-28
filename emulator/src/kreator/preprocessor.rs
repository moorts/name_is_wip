use super::assembler::{get_reserved_names, LABEL_DECL};
use super::parser::eval;
use std::collections::HashMap;
use regex::Regex;

const MACRO_START: &str = "Custom Mac";
const MACRO_END: &str = "Custom End";

pub fn get_preprocessed_code(code: &Vec<String>) -> Result<Vec<String>, &'static str> {
    let decl_regex = Regex::new(LABEL_DECL).unwrap();
    if !has_correct_end(code) {
        return Err("A program must only contain one END statement and it has to be the last");
    }

    let mut set_assignments: HashMap<String, u16> = HashMap::new();
    let mut in_conditional = false;
    let mut condition = false;
    let mut preprocessed_code: Vec<String> = Vec::new();
    let mut pc = 0;

    let equate_assignments = get_equate_assignments(&code)?;
    let code = get_commentless_code(&code);
    let labels = get_labels(&code)?;
    let code = replace_macros(&code)?;

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

        if owned_line.contains("EQU") {
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

fn get_equate_assignments(code: &Vec<String>) -> Result<HashMap<String, u16>, &'static str> {
    let mut assignments: HashMap<String, u16> = HashMap::new();

    for line in code {
        let line = line.trim().to_string();
        if line.contains(" EQU ") {
            let (name, expression) = line.split_once(" EQU ").unwrap();
            if assignments.contains_key(name) {
                return Err("Can't assign a variable more than once using EQU!");
            }
            assignments.insert(name.to_string(), eval_str(expression.to_string()));
        }
    }
    Ok(assignments)
}

pub fn get_line_map(code: &Vec<String>) -> HashMap<u16, usize> {
    let (one_byte_labels, two_byte_labels, three_byte_labels) = get_opc_by_byte_size();
    let label_decl = Regex::new(LABEL_DECL).unwrap();
    
    let mut byte_to_line_map: HashMap<u16, usize> = HashMap::new();
    let mut macro_map = HashMap::new();
    let mut line_index: usize = 0;
    let mut byte_index: u16 = 0;
    let mut in_macro = false;
    let mut in_unmet_conditional = false;
    
    for (index, line) in code.iter().enumerate() {
        let line = label_decl.replace(line, "").trim().to_string();

        if line.contains("ENDM") {
            in_macro = false;
            line_index += 1;
            continue;
        } else if line.contains("ENDIF") {
            in_unmet_conditional = false;
            line_index += 1;
            continue;
        }

        if in_macro || in_unmet_conditional {
            line_index += 1;
            continue;
        }

        if line.contains("IF ") {
            let condition = line.split_once(" ").unwrap().1;
            if eval(condition) == 0 {
                in_unmet_conditional = true;
            }
        }

        // check for macros
        if line.contains(" MACRO") {
            in_macro = true;
            let macro_name = line.split_once(" ").unwrap().0.to_string();
            let mut macro_lines: Vec<String> = Vec::new();
            let mut counter = 1;
            while !get_commentless_code(&vec![code.get(index + counter).unwrap().to_string()]).get(0).unwrap().trim().eq("ENDM") {
                macro_lines.push(code.get(index + counter).unwrap().to_string());
                counter += 1;
            }
            let mut local_map = get_line_map(&macro_lines);
            for value in local_map.values_mut() {
                *value += line_index + 1;
            }
            macro_map.insert(macro_name, local_map);
            line_index += 1;
            continue;
        }

        let operand: &str = if line.contains(" ") {
            line.split_once(" ").unwrap().0
        } else {
            &line
        };
        if macro_map.contains_key(&operand.to_string()) {
            let local_map = macro_map.get(&operand.to_string()).unwrap();
            for (local_byte, line) in local_map {
                byte_to_line_map.insert(local_byte + byte_index, *line);
            }
            byte_index += local_map.len() as u16;
        } else if one_byte_labels.contains(&operand) {
            byte_to_line_map.insert(byte_index, line_index);
            byte_index += 1;
        } else if two_byte_labels.contains(&operand) {
            byte_to_line_map.insert(byte_index, line_index);
            byte_to_line_map.insert(byte_index + 1, line_index);
            byte_index += 2;
        } else if three_byte_labels.contains(&operand) {
            byte_to_line_map.insert(byte_index, line_index);
            byte_to_line_map.insert(byte_index + 1, line_index);
            byte_to_line_map.insert(byte_index + 2, line_index);
            byte_index += 3;
        }
        line_index += 1;
    }
    byte_to_line_map
}

fn get_commentless_code(code: &Vec<String>) -> Vec<String> {
    let comment_regex = Regex::new(r";.*").unwrap();
    let mut new_code = Vec::new();

    for line in code {
        new_code.push(comment_regex.replace_all(line, "").to_string());
    }
    new_code
}

fn eval_str(str: String) -> u16 {
    eval(&str) as u16
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
                macroless_code.push(MACRO_START.to_string());
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
                    let replacement_protection = "@";
                    let mut line = instruction.to_string();

                    for (variable, value) in &input_map {
                        let var_regex = Regex::new(&format!(r"[ ,]{}[ ,+\-*/,].", variable)).unwrap();
                        let end_regex = Regex::new(&format!("[ ,]{} ?$", variable)).unwrap();

                        while let Some(reg_match) = var_regex.find(&line.clone()) {
                            let first_match_symbol = line.get(reg_match.start()..reg_match.start() + 1).unwrap();
                            let last_match_symbol =  line.get(reg_match.end()..).unwrap();
                            let start = match first_match_symbol {
                                " " | "," => reg_match.start() + 1,
                                _ => reg_match.start()
                            };
                            let end = match last_match_symbol {
                                " " | "," | "+" | "-" | "*" | "/" => reg_match.end() - 2,
                                _ => reg_match.end() -1
                            };
                            line.replace_range(start..end - 1, &format!("{}{}", &value, replacement_protection));
                        }
                        if let Some(reg_match) = end_regex.find(&line.clone()) {
                            let first_symbol = line.get(reg_match.start()..reg_match.start() + 1).unwrap();
                            let start = match first_symbol {
                                " " | "," => reg_match.start() + 1,
                                _ => reg_match.start()
                            };
                            line.replace_range(start..reg_match.end(), &format!("{}{}", &value, replacement_protection));
                        }
                    }
                    line = line.replace(replacement_protection, "");
                    macroless_code.push(line.trim().to_string());
                }
                macroless_code.push(MACRO_END.to_string());
                continue 'outer;
            }
        }

        macroless_code.push(owned_line.trim().to_string());
    }
    macroless_code = handle_macro_locals(&macroless_code).unwrap();
    Ok(macroless_code)
}

fn handle_macro_locals(code: &Vec<String>) -> Result<Vec<String>, &'static str> {
    let loc_label_regex = Regex::new(LABEL_DECL).unwrap();
    let glob_label_regex = Regex::new(&format!("{}:", LABEL_DECL)).unwrap();
    let var_name_regex = Regex::new(r"^( *[a-zA-Z@?][a-zA-Z@?0-9]{0,4} )").unwrap();

    let mut generated_label_count: u32 = 0;
    let mut handled_code: Vec<String> = Vec::new();
    let mut label_names: Vec<String> = Vec::new();
    let mut label_map: HashMap<String, String> = HashMap::new();
    let mut equ_names: Vec<String> = Vec::new();
    let mut equ_map: HashMap<String, String> = HashMap::new();
    let mut found_set_names: Vec<String> = Vec::new();
    let mut set_map: HashMap<String, String> = HashMap::new();
    let mut all_existing_names: Vec<String> = Vec::new();
    let mut in_macro = false;

    // search entire code for labels and equ assignments outside of macros
    for line in code {
        if line.eq(MACRO_START) {
            in_macro = true;
            continue;
        }
        if line.eq(MACRO_END) {
            in_macro = false;
            continue;
        }
        if !in_macro && loc_label_regex.is_match(line) {
            let (label, _) = line.split_once(":").unwrap();
            if !label_names.contains(&label.to_string()) {
                label_names.push(label.to_string());
            }
        }
        if !in_macro && line.contains(" EQU ") {
            let (var, _) = line.split_once(" EQU ").unwrap();
            if !var_name_regex.is_match(line) {
                return Err("Illegal variable name!");
            }
            if !equ_names.contains(&var.to_string()) {
                equ_names.push(var.to_string());
            }
        }
    }

    all_existing_names.append(&mut equ_names.clone());
    all_existing_names.append(&mut label_names.clone());

    for line in code {
        let mut owned_line = line.clone();

        // check if current block of code is (not) a macro
        if owned_line.eq(MACRO_START) {
            in_macro = true;
            continue;
        } 
        if owned_line.eq(MACRO_END) {
            in_macro = false;
            label_map.clear();
            equ_map.clear();
            set_map.clear();
            continue;
        }

        // find set assignments outside of macros
        if owned_line.contains(" SET ") && !in_macro {
            let (name, _) = owned_line.split_once(" SET ").unwrap();
            if !var_name_regex.is_match(line) {
                return Err("Illegal variable name!");
            }
            if !found_set_names.contains(&name.to_string()) {
                found_set_names.push(name.to_string());
                all_existing_names.push(name.to_string());
            }
        }

        // change line of macro according to principles of locality
        if in_macro {
            
            // map local labels in macros
            if loc_label_regex.is_match(&owned_line) && !glob_label_regex.is_match(&owned_line) {
                let (label, _) = owned_line.split_once(":").unwrap();
                let gen_name = generate_label_name(&all_existing_names, &mut generated_label_count).unwrap();
                generated_label_count += 1;
                all_existing_names.push(gen_name.clone());
                label_map.insert(label.to_string(), gen_name);
            }

            // convert globally declared label to normal label
            if glob_label_regex.is_match(&owned_line) {
                owned_line = owned_line.replace("::", ":");
            }

            // map local equ assignments
            if line.contains(" EQU ") {
                let (name, _) = owned_line.split_once(" EQU ").unwrap();
                let gen_name = generate_label_name(&all_existing_names, &mut generated_label_count).unwrap();
                generated_label_count += 1;
                all_existing_names.push(gen_name.clone());
                equ_map.insert(name.to_string(), gen_name);
            }

            // replace local label calls and equ assigned variables
            for (old_label, generated_label) in &label_map {
                owned_line = owned_line.replace(old_label, generated_label);
            }
            for (old_name, new_name) in &equ_map {
                owned_line = owned_line.replace(old_name, new_name);
            }
            
            // check if set assignment variable existed outside of macro
            if owned_line.contains(" SET ") {
                let (name, _) = owned_line.split_once(" SET ").unwrap();
                if !found_set_names.contains(&name.to_string()) {
                    let gen_name = generate_label_name(&all_existing_names, &mut generated_label_count).unwrap();
                    generated_label_count += 1;
                    all_existing_names.push(gen_name.clone());
                    set_map.insert(name.to_string(), gen_name);
                }
            }
            for (old_name, new_name) in &set_map {
                owned_line = owned_line.replace(old_name, new_name);
            }
        }
        handled_code.push(owned_line.to_string());
    }
    Ok(handled_code)
}

fn generate_label_name(taken_names: &Vec<String>, generated_label_count: &mut u32) -> Result<String, &'static str> {
    loop {
        let label_char = char::from_u32((*generated_label_count / 10000) as u32 + 'A' as u32).unwrap();
        let label_num = (*generated_label_count % 10000) as u32;

        if label_char.eq(&'[') {
            return Err("Exceeded maximum amount of local labels!")
        }
        let new_label = format!("{}{}", label_char, label_num);
        if !taken_names.contains(&new_label) {
            return Ok(new_label)
        }
        *generated_label_count += 1;
    }
}

fn get_labels(code: &Vec<String>) -> Result<HashMap<String, u16>, &'static str> {
    let label_regex = Regex::new(LABEL_DECL).unwrap();
    let (one_byte_labels, two_byte_labels, three_byte_labels) = get_opc_by_byte_size();
    let mut reserved_names = vec![
        "ORG", "EQU", "SET", "END", "IF", "ENDIF", "MACRO", "ENDM", "B", "C", "D", "H", "L", "A", "SP", "PSW"
    ];
    reserved_names.extend(&one_byte_labels);
    reserved_names.extend(&two_byte_labels);
    reserved_names.extend(&three_byte_labels);

    let mut temp_labels = Vec::new();
    let mut labels = HashMap::new();
    let mut mem_address: u16 = 0;

    for line in code {
        if line.starts_with("ORG ") {
            mem_address = eval(line.split_once("ORG ").unwrap().1) as u16;
        }
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
                        labels.insert(String::from(new_label), mem_address);
                    }
                }
                let line = label_regex.replace(line, "").trim().to_string();
                mem_address += get_byte_amount_of_line(&line);
            }
        } else {
            while let Some(new_label) = temp_labels.pop() {
                if labels.contains_key(&new_label) {
                    return Err("label must not be assigned twice!");
                } else {
                    labels.insert(String::from(new_label), mem_address);
                }
            }
            mem_address += get_byte_amount_of_line(&line);
        }
    }
    if !temp_labels.is_empty() {
        return Err("labels must not point to an empty address!");
    }
    Ok(labels)
}

fn get_opc_by_byte_size() -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {
    (vec![
        "NOP", "STAX", "INX", "INR", "DCR", "RLC", "DAD", "LDAX", "DCX", "RRC", "RAL", "RAR",
        "DAA", "STC", "CMC", "HLT", "SBB", "MOV", "ANA", "XRA", "ORA", "CMP", "RNZ", "POP",
        "PUSH", "RST", "RZ", "RET", "RNC", "RC", "RPO", "XTHL", "RPE", "PCHL", "XCHG", "RP",
        "DI", "RM", "SPHL", "EI", "CMA", "ADD", "ADC", "SUB"
    ],
    vec![
        "MVI", "ADI", "ACI", "OUT", "SUI", "IN", "SBI", "ANI", "XRI", "ORI", "CPI"
    ],
    vec![
        "LXI", "SHLD", "LHLD", "STA", "LDA", "JNZ", "JMP", "CNZ", "JZ", "CZ", "CALL", "JNC",
        "CNC", "JC", "CC", "JPO", "CPO", "JPE", "CPE", "JP", "CP", "JM", "CM"
    ])
}

fn get_byte_amount_of_line(line: &String) -> u16 {
    let (one_byte_labels, two_byte_labels, three_byte_labels) = get_opc_by_byte_size();
    if !line.trim().is_empty() {
        let opc = line.trim().split(" ").next().unwrap();
        if one_byte_labels.contains(&opc) {
            return 1;
        } else if two_byte_labels.contains(&opc) {
            return 2;
        } else if three_byte_labels.contains(&opc) {
            return 3;
        }
    }
    0
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
    fn remove_comments() {
        let ppc = get_commentless_code(&convert_input(vec![";comment\nMOV A, B;asdf\n;END;\nEND"]));
        let expected = convert_input(vec!["\nMOV A, B\n\nEND"]);

        assert_eq!(ppc, expected);
    }

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

        let code = &convert_input(vec!["MAC MACRO p1, p2", "ADI p1", "ADI p2", "ENDM", "MAC p2, 5"]);
        let ppc = replace_macros(code);
        assert_eq!(Ok(convert_input(vec!["ADI p2", "ADI 5"])), ppc);
    }

    #[test]
    fn labels_in_macros() {
        let code = convert_input(vec![MACRO_START, "LOOP:", "MOV A,B", "JMP LOOP", MACRO_END, MACRO_START, "LOOP:", "MOV A,B", "JMP LOOP", MACRO_END]);
        let ppc = handle_macro_locals(&code).unwrap();
        assert_eq!(ppc[0], "A0:");
        assert_eq!(ppc[2], "JMP A0");
        assert_eq!(ppc[3], "A1:");

        let code = convert_input(vec!["@LAB:", "MOV A,B", MACRO_START, "@LAB: JMP @LAB", MACRO_END]);
        let ppc = handle_macro_locals(&code).unwrap();
        assert_eq!(ppc[0], "@LAB:");
        assert_eq!(ppc[2], "A0: JMP A0");

        let code = convert_input(vec!["GLOB: MOV A,B", MACRO_START, "GLOB2::", "NOP", "JMP GLOB2", MACRO_END]);
        let ppc = handle_macro_locals(&code).unwrap();
        assert_eq!(ppc[1], "GLOB2:");
        assert_eq!(ppc[3], "JMP GLOB2");

        let code = convert_input(vec!["A0: MOV A,B", MACRO_START, "LAB:", "MOV A,B", MACRO_END]);
        let ppc = handle_macro_locals(&code).unwrap();
        assert_eq!(ppc[1], "A1:");

        let code = convert_input(vec!["A2: JMP A1", MACRO_START, "LAB:", "JMP LAB", MACRO_END, "A0:", "MOV A,B"]);
        let ppc = handle_macro_locals(&code).unwrap();
        assert_eq!(ppc[1], "A1:");
    }

    #[test]
    fn variables_in_macros() {
        let code = convert_input(vec!["VAL EQU 6", MACRO_START, "VAL EQU 8", "DB VAL", MACRO_END, "JMP VAL"]);
        let ppc = handle_macro_locals(&code).unwrap();
        assert!(ppc[0].contains("VAL"));
        assert_eq!(ppc[1], "A0 EQU 8");
        assert_eq!(ppc[2], "DB A0");
        assert!(ppc[3].contains("VAL"));

        let code = convert_input(vec!["VAL SET 5", MACRO_START, "VAL SET 8", MACRO_END]);
        let ppc = handle_macro_locals(&code).unwrap();
        assert!(ppc[1].eq("VAL SET 8"));

        let code = convert_input(vec!["TEST SET 5", MACRO_START, "VAL SET 8", MACRO_END]);
        let ppc = handle_macro_locals(&code).unwrap();
        assert_eq!(ppc[1], "A0 SET 8");
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
    fn label_bytes() {
        let code = convert_input(vec!["MVI B,10", "start: ADD B", "DCR B", "JNZ start", "MOV B,A", "HLT", "END"]);
        let mut labels = HashMap::new();
        labels.insert("start".to_string(), 2);

        assert_eq!(Ok(labels), get_labels(&code));

        let code = convert_input(vec!["ORG 5+5", "MVI B,10", "start: ADD B", "DCR B", "JNZ start", "MOV B,A", "ORG 0A1H", "test: HLT", "END"]);
        let mut labels = HashMap::new();
        labels.insert("start".to_string(), 12);
        labels.insert("test".to_string(), 161);

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
    fn line_mapping() {
        let code = convert_input(vec!["MOV A,B", "", "JMP 1", "label:", "lab:", "MVI D, 3H"]);
        let mut map = HashMap::new();

        map.insert(0, 0);
        map.insert(1, 2);
        map.insert(2, 2);
        map.insert(3, 2);
        map.insert(4, 5);
        map.insert(5, 5);

        assert_eq!(get_line_map(&code), map);

        let code = convert_input(vec!["mac MACRO", "", "MOV A,B", "ENDM", "label:", "mac", "", "JMP 0", "mac"]);
        map.clear();

        map.insert(0, 2);
        map.insert(1, 7);
        map.insert(2, 7);
        map.insert(3, 7);
        map.insert(4, 2);

        assert_eq!(get_line_map(&code), map);
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
            "IF VAR1",
            "EI",
            "ENDIF",
            "END",
            "",
        ]);

        let result = convert_input(vec!["JMP 0 +6", "ADD C", "POP B", "RZ", "EI"]);

        assert_eq!(Ok(result), get_preprocessed_code(&code));

        let mut map = HashMap::new();
        map.insert(0, 1);
        map.insert(1, 1);
        map.insert(2, 1);
        map.insert(3, 2);
        map.insert(4, 8);
        map.insert(5, 11);
        map.insert(6, 16);

        //assert_eq!(map, get_line_map(&code));
    }

    fn convert_input(lines: Vec<&str>) -> Vec<String> {
        let mut string_vector: Vec<String> = Vec::new();
        for line in lines {
            string_vector.push(line.to_string())
        }
        string_vector
    }
}
