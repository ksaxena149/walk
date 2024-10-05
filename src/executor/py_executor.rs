use std::process::{Command, Stdio};

pub fn execute_py(filename: String, file_stem: &str) -> Result<(), String> {
    let output = Command::new("python")
        .arg(filename)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to start Python interpreter:\n{}", e))?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Runtime Error: Script exited with status code {}", output.status.code().unwrap_or(-1)));
    }
    Ok(())
}