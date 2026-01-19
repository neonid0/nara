use std::env;
use std::fs;
use std::io::{self, Write};

// cli for nara-lang
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check if a file argument was provided
    if args.len() > 1 {
        // File execution mode
        let filename = &args[1];

        match execute_file(filename) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // REPL mode
        repl()
    }
}

fn execute_file(filename: &str) -> Result<(), String> {
    // Read file contents
    let contents = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;

    // Trim the contents to remove trailing whitespace
    let contents = contents.trim();

    // Skip if file is empty or only contains comments/whitespace
    if contents.is_empty() {
        return Ok(());
    }

    // Create environment
    let mut env = nara::Env::default();

    // Parse and execute
    let parse =
        nara::parse(contents).map_err(|msg| format!("Parse error in '{}': {}", filename, msg))?;

    parse
        .eval(&mut env)
        .map_err(|msg| format!("Evaluation error in '{}': {}", filename, msg))?;

    Ok(())
}

fn repl() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    let mut env = nara::Env::default();

    writeln!(stdout, "Nara REPL v{}", env!("CARGO_PKG_VERSION"))?;
    writeln!(stdout, "Type your expressions below. Press Ctrl+D to exit.")?;
    writeln!(stdout)?;

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        let bytes_read = stdin.read_line(&mut input)?;

        // Break on EOF (when read_line returns 0 bytes)
        if bytes_read == 0 {
            writeln!(stdout)?;
            break Ok(());
        }

        match run(input.trim(), &mut env) {
            Ok(Some(val)) => {
                writeln!(stdout, "{:?}", val)?;
            }
            Ok(None) => {}
            Err(msg) => writeln!(stderr, "{}", msg)?,
        }

        input.clear();
    }
}

fn run(input: &str, env: &mut nara::Env) -> Result<Option<nara::Val>, String> {
    let parse = nara::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

    let evaluated = parse
        .eval(env)
        .map_err(|msg| format!("Evaluation error: {}", msg))?;

    if evaluated == nara::Val::Unit {
        Ok(None)
    } else {
        Ok(Some(evaluated))
    }
}
