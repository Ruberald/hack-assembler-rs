use std::collections::HashMap;

fn label_map(op: &str) -> Option<&str> {
    match op {
        "SP" => Some("0"),
        "LCL" => Some("1"),
        "ARG" => Some("2"),
        "THIS" => Some("3"),
        "THAT" => Some("4"),
        "R0" => Some("0"),
        "R1" => Some("1"),
        "R2" => Some("2"),
        "R3" => Some("3"),
        "R4" => Some("4"),
        "R5" => Some("5"),
        "R6" => Some("6"),
        "R7" => Some("7"),
        "R8" => Some("8"),
        "R9" => Some("9"),
        "R10" => Some("10"),
        "R11" => Some("11"),
        "R12" => Some("12"),
        "R13" => Some("13"),
        "R14" => Some("14"),
        "R15" => Some("15"),
        "SCREEN" => Some("16384"),
        "KBD" => Some("24576"),
        _ => None
    }
}

pub fn decode(mut tokens: Vec<String>) -> Vec<String> {
    let mut sym_map: HashMap<String, i32> = HashMap::new();
    let mut var_count: i32 = 15;

    let mut line_index: i32 = 0;

    // First pass
    tokens
        .iter()
        .for_each(|token| match token.chars().next().unwrap() {
            '(' => {
                let mut t = token.clone();
                t.remove(0); t.remove(t.len() - 1);
                sym_map.insert(t, line_index as i32);
            },
            _ => line_index += 1
        });

    // Second pass
    tokens = tokens
        .into_iter()
        .map(|token| match token.chars().next().unwrap() {
            '@' => if token.chars().skip(1).next().unwrap().is_numeric() {
                token
            } else {
                match label_map(&token.replace('@', "")) {
                    Some(s) => format!("@{}", s),
                    None => match sym_map.get(&token.replace('@', "")) {
                        Some(index) => format!("@{}", index),
                        None => {
                            var_count = var_count + 1;
                            sym_map.insert(token.clone().replace('@', ""), var_count);
                            format!("@{}", var_count)
                        }
                    }
                }
            }
            '(' => "".to_string(),
            _ => token
        })
    .filter(|s| !s.is_empty())
        .collect();

            tokens
}
