use bfvm::interpret;
use std::{env, fs, io::Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: cargo run -r -q -- <filepath>");
        return Ok(());
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)?;
    match interpret(&contents) {
        Err(e) => eprintln!("{e}"),
        _ => {}
    }

    Ok(())
}
