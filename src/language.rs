use std::path::Path;
use crate::executor;

pub fn execute_program(filename: &str) -> Result<(), String> {
    let lang = Path::new(&filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown");
    match lang {
        "c" => {
            let file_stem = Path::new(&filename)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("unknown");
            executor::c_executor::execute_c(filename.to_string(), file_stem)?;
        }
        "py" => {
            let file_stem: &str = Path::new(&filename)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("unknown");
            executor::py_executor::execute_py(filename.to_string(), file_stem)?;
        }
        _ => {
            return Err("Language not supported.".to_string())
        }
    }
    Ok(())
}