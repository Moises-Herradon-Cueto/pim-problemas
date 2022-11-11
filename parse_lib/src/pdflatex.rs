use std::{fs, path::Path, process::Command};

pub fn run<P: AsRef<Path>>(path: P) {
    let entries = fs::read_dir(path).unwrap();
    let mut count = 0_u8;
    for file in entries {
        let file = file.unwrap();
        if file.file_name().to_string_lossy() != "pim.sty" {
            let path = file.path();
            let mut command = Command::new("pdflatex");
            command.arg("-interaction=nonstopmode").arg(path);
            let result = command.output().expect("Failed to run");

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
}
