use std::fs;

pub fn read_file(file_name: &str) -> Vec<String> {
    let file = fs::read_to_string(file_name).unwrap_or(String::new());

    file
        .lines()
        .filter(|l| !(l.chars().nth(0) == Some('/') && l.chars().nth(1) == Some('/') || l == &""))
        .flat_map(|s| s.splitn(2, "//").take(1))
        .map(|s| s.replace(' ', "").to_string())
        .filter(|s| s != &"")
        .collect()
}
