use std::{fmt::Display, str::FromStr};

use regex::{Captures, Regex};

use crate::Data;

const REDUNDANT: [&str; 14] = [
    "inputenc", "babel", "pim", "graphicx", "amssymb", "latexsym", "amsmath", "amsthm", "verbatim",
    "gensymb", "mathrsfs", "pgfplots", "textcomp", "tikz",
];

/// .
///
/// # Panics
///
/// Panics if any little thing goes wrong
/// while parsing
pub fn clean(data: &mut Data) {
    let packages = data.paquetes.split('\n');
    let packages: Vec<_> = packages
        .flat_map(|package| {
            let pieces = package.split(',');
            pieces.filter_map(|piece| {
                let package: Statement = piece.parse().unwrap();
                if matches!(package, Statement::Redundant) {
                    None
                } else {
                    Some(package.to_string())
                }
            })
        })
        .collect();
    data.paquetes = packages.join("\n");
}

#[derive(PartialEq, Eq)]
enum Statement {
    UsePackage(String),
    UsePackageOption(String, String),
    UseTikzLibrary(String),
    PgfPlotsSet(String),
    NewCommand(String, Option<u8>, String),
    DeclareMathOperator(String, String),
    Redundant,
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UsePackage(package) => write!(f, "\\usepackage{{{package}}}"),
            Self::UsePackageOption(option, package) => {
                write!(f, "\\usepackage[{option}]{{{package}}}")
            }
            Self::UseTikzLibrary(library) => write!(f, "\\usetikzlibrary{{{library}}}"),
            Self::PgfPlotsSet(set) => write!(f, "\\pgfplotsset{{{set}}}"),
            Self::NewCommand(def, n, out) => {
                if let Some(n) = n {
                    write!(f, "\\newcommand{{{def}}}[{n}]{{{out}}}")
                } else {
                    write!(f, "\\newcommand{{{def}}}{{{out}}}")
                }
            }
            Self::DeclareMathOperator(def, out) => {
                write!(f, "\\DeclareMathOperator{{{def}}}{{{out}}}")
            }
            Self::Redundant => write!(f, ""),
        }
    }
}

impl FromStr for Statement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let set = [
            Regex::new(r"\\usepackage(\[\s*(.*?)\s*\])?\{\s*([^,]*?)\s*\}").unwrap(),
            Regex::new(r"\\usetikzlibrary\{\s*([^,]*?)\s*\}").unwrap(),
            Regex::new(r"\\pgfplotsset\{\s*([^,]*?)\s*\}").unwrap(),
            Regex::new(r"\\newcommand\{\s*([^,]*?)\s*\}(\[\s*(.*?)\s*\])?\{\s*([^,]*?)\s*\}")
                .unwrap(),
            Regex::new(r"\\DeclareMathOperator\{\s*([^,]*?)\s*\}\{\s*([^,]*?)\s*\}").unwrap(),
        ];
        if let Some(captures) = set[0].captures(s) {
            let option = get_n_optional(&captures, 2);
            let package = get_n(&captures, 3);
            if let Some(option) = option {
                return Ok(Self::UsePackageOption(option, package));
            }
            if REDUNDANT.contains(&package.as_str()) {
                return Ok(Self::Redundant);
            }
            return Ok(Self::UsePackage(package));
        }
        if let Some(captures) = set[1].captures(s) {
            let library = get_n(&captures, 1);
            return Ok(Self::UseTikzLibrary(library));
        }
        if let Some(captures) = set[2].captures(s) {
            let plots_set = get_n(&captures, 1);
            return Ok(Self::PgfPlotsSet(plots_set));
        }
        if let Some(captures) = set[3].captures(s) {
            let command = get_n(&captures, 1);
            let args: Option<u8> = get_n_optional(&captures, 3).map(|x| x.parse().unwrap());
            let output = get_n(&captures, 4);
            return Ok(Self::NewCommand(command, args, output));
        }
        if let Some(captures) = set[4].captures(s) {
            let command = get_n(&captures, 1);
            let output = get_n(&captures, 2);
            return Ok(Self::DeclareMathOperator(command, output));
        }
        Err(String::from(s))
    }
}

fn get_n(captures: &Captures, n: usize) -> String {
    captures.get(n).unwrap().as_str().to_owned()
}

fn get_n_optional(captures: &Captures, n: usize) -> Option<String> {
    captures.get(n).map(|x| x.as_str().to_owned())
}
