use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

use regex::Regex;

use crate::{Db, Fields};
use colored::Colorize;

/// .
///
/// # Errors
///
/// This function will return an error if
/// the regex is wrong
pub fn apply<H>(
    regex: &str,
    replacement: &str,
    field: Option<Fields>,
    columns: Option<u16>,
    data: &mut Db<H>,
) -> Result<(), String> {
    let field = field.unwrap_or(Fields::Problem);
    let regex = Regex::new(regex).map_err(|err| format!("Error in the regex: {err}"))?;
    for d in data.values_mut() {
        let info = field.get_string(d);
        let new_info = regex.replace_all(&info, replacement);
        let parsed_info = field.parse(&new_info);
        let Ok(parsed_info) =parsed_info else {
            println!("{}\n{new_info}\n{parsed_info:?}","Error parsing. Got".red().bold());
            continue;
        };
        if field.get(d).to_owned() != parsed_info {
            fs::write("/tmp/file_1", info.as_ref()).expect("Couldn't write to tmp");
            fs::write("/tmp/file_2", new_info.as_ref()).expect("Couldn't write to tmp");
            let mut command = Command::new("delta");
            command
                .arg("--side-by-side")
                .arg("--wrap-max-lines=unlimited");
            columns.map(|col| command.arg(format!("--width={col}")));
            command.arg("/tmp/file_1").arg("/tmp/file_2");
            // println!("{}\n{command:#?}", "Command:".red().bold());

            let diff = command.output().expect("Failed to run delta");
            if !diff.stderr.is_empty() {
                println!(
                    "{}{}",
                    "Exited with error:".red().bold(),
                    String::from_utf8_lossy(&diff.stderr)
                );
            }
            let diff = String::from_utf8_lossy(&diff.stdout);
            println!("{diff}");
            print!("Replace? (y/n/stop)");
            // print!(
            //     "{}\n{info}\n\n{}\n{new_info}\n\nReplace? (y/n/stop)",
            //     "Old:".red().bold(),
            //     "New:".red().bold()
            // );
            let stdin = io::stdin();
            loop {
                io::stdout()
                    .flush()
                    .map_err(|err| format!("Error flushing, ew:{err}"))?;
                let mut user_input = String::new();
                stdin
                    .read_line(&mut user_input)
                    .map_err(|err| format!("Error reading: {err}"))?;
                match user_input.as_str().trim() {
                    "y" | "Y" => {
                        parsed_info.set(d);
                        break;
                    }
                    "n" | "N" => {
                        break;
                    }
                    "stop" => return Ok(()),
                    _ => {
                        println!("Enter y/n/stop");
                    }
                }
            }
        }
    }

    Ok(())
}

#[must_use]
pub fn parse_file(path: &Path) -> Vec<(String, String, Option<Fields>)> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let mut output = vec![];
    for line in contents.lines() {
        let mut pieces = line.split("---");
        let Some(regex) = pieces.next() else {
            println!("Line has no beginning: {line}");
            continue;
        };
        let Some(replacement) = pieces.next() else {
            println!("Line has no replacement: {line}");
            continue;
        };
        let field: Option<Fields> = pieces
            .next()
            .map(|s| s.parse().expect("Failed to parse field"));
        output.push((regex.to_owned(), replacement.to_owned(), field));
    }
    output
}
