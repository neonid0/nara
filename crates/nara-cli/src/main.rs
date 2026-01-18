use std::io::{self, Write};

// cli for nara-lang
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    let mut env = nara::Env::default();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        let bytes_read = stdin.read_line(&mut input)?;

        // Break on EOF (when read_line returns 0 bytes)
        if bytes_read == 0 {
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
