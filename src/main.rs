use bfvm::{interpret, Memory, MEM_SIZE};
use std::io::{stdin, stdout, Result};
use std::{env, fs};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: cargo run -r -q -- <filepath>");
        return Ok(());
    }

    let file_path = &args[1];
    let input = fs::read_to_string(file_path)?;

    let mut memory: Memory = [0; MEM_SIZE];

    let stdin = stdin().lock();
    let stdout = stdout().lock();
    if let Err(e) = interpret(&input, &mut memory, stdin, stdout) {
        eprintln!("{e}");
    }

    Ok(())
}
