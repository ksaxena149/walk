use std::{env, fmt::format, path::Path, process::Command};


fn main() {
    let filename = parse_args().expect("Filename not provided");
    let lang = Path::new(&filename).extension().and_then(|ext| ext.to_str()).unwrap_or("unknown");
    match lang {
        "c" => {
            let file = Path::new(&filename).file_stem().and_then(|stem| stem.to_str()).unwrap_or("unknown");
            match execute_c(filename.clone(), file.to_string()) {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("Error executing C program:\n{}", e)
            }
        },
        _ => println!("Language not supported :(")
    }
}

fn parse_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: walk <filename>");
        return None
    }
    Some(args[1].clone())
}

fn execute_c(filename: String, file_stem: String) -> Result<String, String> {
    let output = Command::new("gcc")
        .args(&[&filename, "-o", &format!("{}.exe", file_stem)])
        .output()
        .map_err(|e| format!("Failed to execute gcc: {}", e))?;
    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Compilation failed:\n{}", err_msg));
    }
    
    let mut command = Command::new(format!(".\\{}.exe", file_stem));
    command.stdin(std::process::Stdio::inherit()); // Pass stdin from the parent process
    command.stdout(std::process::Stdio::inherit()); // Pass stdout to the parent process
    command.stderr(std::process::Stdio::inherit() ); // Pass stderr to the parent process

    // Execute the compiled executable
    let status = command.status().map_err(|e| format!("Failed to execute program: {}", e))?;

    // Check if the executable ran successfully
    if !status.success() {
        return Err(format!("Runtime error: Program exited with status code {}", status.code().unwrap_or(-1)));
    }

    let program_output = String::from_utf8_lossy(&output.stdout);
    Ok(program_output.to_string())
}
