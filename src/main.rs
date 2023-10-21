use std::{env, fs, path::Path};

mod lexer;
mod assembler;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Provide file name as single argument!");
    }

    let file_name = &args[1];
    println!("{} contents:", file_name);

    let tokens: Vec<String> = lexer::read_file(file_name);
    println!("{:#?}", tokens);

    println!("\nAssembled file: ");
    let assembled = assembler::assemble(tokens);
    println!("{:#?}", assembled);

    let file_path = Path::new(file_name).with_extension("hack");
    let file_path_name = file_path.to_str().unwrap().to_string();
    fs::write(file_path, assembled.join("\n"))
        .expect("Unable to write binary.");
    println!("Binary written to {}", file_path_name);

    Ok(())
}
