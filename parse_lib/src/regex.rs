use std::io::{self, Write};

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
    data: &mut Db<H>,
) -> Result<(), String> {
    let field = field.unwrap_or(Fields::Problem);
    println!("Entered regex: {regex}");
    println!("Entered replacement: {replacement}");
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
            print!(
                "{}\n{info}\n\n{}\n{new_info}\n\nReplace? (y/n/stop)",
                "Old:".red().bold(),
                "New:".red().bold()
            );
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
