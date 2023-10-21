fn comp_map(op: &str) -> &str {
    match op {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001111",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        _ => op
    }
}

fn dest_map(op: &str) -> &str {
    match op {
        "" => "000",
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => op
    }
}

fn jmp_map(op: &str) -> &str {
    match op {
        "" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => op
    }
}

fn map_operands(operation: String) -> String {
    let mut dest = String::from("");
    let mut comp = String::from("");
    let mut jmp = String::from("");

    let mut dest_over = false;
    let mut comp_over = true;
    let mut jmp_over = true;

    for ch in operation.chars() {
        match ch {
            '=' => {
                dest_over = true;
                comp_over = false;
            },

            ';' => {
                if !dest_over {
                    dest_over = true;
                    comp = dest;
                    dest = String::from("");
                }
                comp_over = true;
                jmp_over = false;
            }

            _ => {
                if !dest_over {
                    dest.push(ch);
                }
                else if !comp_over {
                    comp.push(ch);
                }
                else if !jmp_over {
                    jmp.push(ch)
                }
            }
        }
    }

    format!("{}{}{}", comp_map(&comp), dest_map(&dest), jmp_map(&jmp))
}

fn convert_token(token: &String) -> String {
    match token.chars().next().unwrap() {
        '@' => {
            format!("{:016b}", (token.clone().replace('@', "").parse::<u16>().unwrap()))
        }
        _ => {
            format!("111{}", map_operands(token.clone()))
        }
    }
}

pub fn assemble(tokens: Vec<String>) -> Vec<String> {
    tokens.iter()
        .map(|token| convert_token(token))
        .collect()
}
