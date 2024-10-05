use std::process::{Command, Stdio};

pub fn execute_cpp(file_stem: &str) -> Result<(), String> {
    // compile CPP program
    let output = Command::new("g++")
        .args(&[&format!("{}.cpp", file_stem), "-o", &format!("{}.exe", file_stem)])
        .output()
        .map_err(|e| format!("Failed to execute g++: {}", e))?;
    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Compilation failed:\n{}", err_msg));
    }
    
    let mut command = Command::new(format!(".\\{}.exe", file_stem));
    command.stdin(Stdio::inherit()); // Pass stdin from the parent process
    command.stdout(Stdio::inherit()); // Pass stdout to the parent process
    command.stderr(Stdio::inherit() ); // Pass stderr to the parent process

    // Execute the compiled executable
    let status = command.status().map_err(|e| format!("Failed to execute program: {}", e))?;

    // Check if the executable ran successfully
    if !status.success() {
        return Err(format!("Runtime error: Program exited with status code {}", status.code().unwrap_or(-1)));
    }

    Ok(())
}