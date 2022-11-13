use std::{fs, io, path::Path, process::Command};

/// # Errors
///
/// This function will return an error if
/// there is an IO error
pub fn run<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    let entries = fs::read_dir(path)?;
    let mut count = 0_u8;
    for file in entries {
        let Ok(file) = file else {
            println!("Failed to open file {file:?}");
            continue;
        };
        if file.file_name().to_string_lossy() != "pim.sty" {
            let path = file.path();
            let mut command = Command::new("pdflatex");
            command.arg("-interaction=nonstopmode").arg(path);
            let result = command.output()?;

            if !result.status.success() {
                println!(
                    "No se ha compilado bien el {}",
                    file.file_name().to_string_lossy()
                );
                count += 1;
            }
        }
    }
    println!("{count} fracasos");
    Ok(())
}
