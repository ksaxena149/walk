use std::path::Path;
use crate::executor;

pub fn execute_program(filename: &str) -> Result<(), String> {
    let lang = Path::new(&filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown");
    let file_stem = Path::new(&filename)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown");
    match lang {
        "c" => {
            executor::c_executor::execute_c(file_stem)?;
        }
        "py" => {
            executor::py_executor::execute_py(file_stem)?;
        }
        "cpp" => {
            executor::cpp_executor::execute_cpp(file_stem)?;
        }
        "java" => {
            executor::java_executor::execute_java(file_stem)?;
        }
        _ => {
            return Err("Language not supported.".to_string())
        }
    }
    Ok(())
}