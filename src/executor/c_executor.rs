use std::process::{Command, Stdio};

pub fn execute_c(filename: String, file_stem: &str) -> Result<(), String> {
    // compile C program
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

    Ok(())
}