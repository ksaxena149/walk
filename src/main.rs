mod args;
mod language;
mod executor;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), String> {
    let filename = args::parse_args()?;
    language::execute_program(&filename)?;
    Ok(())
}