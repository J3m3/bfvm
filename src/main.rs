use bfvm::{interpret, jit_compile, Memory, MEM_SIZE};
use std::io::{stdin, stdout, Result};
use std::{env, fs, mem};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 4 {
        eprintln!("USAGE: cargo run -r -q -- <filepath> [--no-jit]");
        return Ok(());
    }

    // TODO: make order insensitive
    let file_path = &args[1];
    let jit_off = args.get(2).map(|option| &option[..]) == Some("--no-jit");
    let input = fs::read_to_string(file_path)?;

    let mut memory: Memory = [0; MEM_SIZE];
    if jit_off {
        let stdin = stdin().lock();
        let stdout = stdout().lock();
        if let Err(e) = interpret(&input, &mut memory, stdin, stdout) {
            eprintln!("{e}");
        }
    } else {
        match jit_compile(&input, &mut memory) {
            Ok(code) => unsafe {
                (mem::transmute::<*const u8, extern "C" fn()>(code.as_ptr()))();
            },
            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}
