use bfvm::interpret;
use std::io::{stdin, stdout};
use std::{env, fs, io::Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: cargo run -r -q -- <filepath>");
        return Ok(());
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)?;

    let stdin = stdin().lock();
    let stdout = stdout().lock();
    match interpret(&contents, stdin, stdout) {
        Err(e) => eprintln!("{e}"),
        _ => {}
    }

    Ok(())
}
