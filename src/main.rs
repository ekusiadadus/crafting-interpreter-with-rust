use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        if let Err(err) = run_file(&args[1]) {
            eprintln!("Error running script: {}", err);
            process::exit(1);
        }
    } else {
        if let Err(err) = run_prompt() {
            eprintln!("Error running prompt: {}", err);
            process::exit(1);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    run(&contents)?;
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        print!("> ");
        stdout.flush()?;

        buffer.clear();
        stdin.lock().read_line(&mut buffer)?;

        run(&buffer)?;
    }
}

fn run(source: &str) -> io::Result<()> {
    // TODO: Implement interpreter logic here
    println!("Running: {}", source);
    Ok(())
}
