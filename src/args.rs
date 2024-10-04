use std::env;

pub fn parse_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: walk <filename>".to_string());
    }
    Ok(args[1].clone())
}