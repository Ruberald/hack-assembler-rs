use std::fs;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Provide file name as single argument!");
    }

    let file_name = &args[1];
    println!("{} contents:", file_name);
    println!("{}", fs::read_to_string(file_name).unwrap_or(String::new()));

    Ok(())
}
