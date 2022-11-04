use crate::data::Data;

use crate::preamble::into_template;
use crate::process_tex::find_year;
use regex::Regex;

use ParseResult::{Template, ToChange};

pub enum ParseResult {
    Template,
    ToChange(String),
}

/// .
///
/// # Panics
///
/// Panics if I mess up.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn string_and_data(input: &str, data: &mut Data) -> ParseResult {
    if is_template(input, data) {
        return Template;
    }

    let problem_regex = Regex::new(r"(?s)\\begin\{ejer\}(.*)\\end\{ejer\}").expect("regex wrong");
    let problem = problem_regex
        .captures_iter(input)
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    data.enunciado = problem.to_owned();

    let sol_regex = [
        Regex::new(r"(?s)\\begin\{proof\}\[Solución\](.*)\\end\{proof\}").expect("regex wrong"),
        Regex::new(r"(?s)\{\\bf Soluci\\'on:\}(.*)\\end\{document\}").expect("regex wrong"),
        Regex::new(r"(?s)\{\\bf Solución:\}(.*)\\end\{document\}").expect("regex wrong"),
    ];

    let solution = sol_regex
        .iter()
        .flat_map(|regex| regex.captures_iter(input))
        .next()
        .ok_or(format!("{data:#?}"))
        .expect("Didn't find solution")
        .get(1)
        .unwrap()
        .as_str();

    let paquetes_1: String = Regex::new(r"\\usepackage\[(.*)\]\{(.*)}")
        .expect("messed up")
        .captures_iter(input)
        .filter_map(|result| {
            let option = result.get(1).unwrap().as_str();
            let package = result.get(2).unwrap().as_str();
            if [
                "inputenc", "babel", "pim", "graphicx", "amssymb", "latexsym", "amsmath", "amsthm",
                "verbatim",
            ]
            .contains(&package)
            {
                return None;
            }
            Some(format!("\\usepackage[{option}]{{{package}}}\n"))
        })
        .collect();

    let paquetes_2: String = Regex::new(r"\\usepackage\{(.*)}")
        .expect("messed up")
        .captures_iter(input)
        .flat_map(|result| {
            let packages = result.get(1).unwrap().as_str().split(',');
            packages
                .filter(|package| {
                    ![
                        "inputenc", "babel", "pim", "graphicx", "amssymb", "latexsym", "amsmath",
                        "amsthm", "verbatim",
                    ]
                    .contains(package)
                })
                .map(|package| format!("\\usepackage{{{package}}}\n"))
        })
        .collect();

    let tikz_libraries: String = Regex::new(r"\\usetikzlibrary\{(.*)}")
        .expect("messed up")
        .captures_iter(input)
        .map(|result| {
            let package = result.get(1).unwrap().as_str();
            format!("\\usetikzlibrary{{{package}}}\n")
        })
        .collect();
    let pgfplotsets: String = Regex::new(r"\\pgfplotsset\{(.*)}")
        .expect("messed up")
        .captures_iter(input)
        .map(|result| {
            let package = result.get(1).unwrap().as_str();
            format!("\\pgfplotsset{{{package}}}\n")
        })
        .collect();
    let mut temas = data.temas.join(", ");

    if temas.is_empty() {
        temas = "%".into();
    }

    let id = data.id;

    let mut fuente = &data.fuente;
    let percent = String::from("%");

    if fuente.is_empty() {
        fuente = &percent;
    }
    let mut comentarios = data.comentarios.join(", ");

    if comentarios.is_empty() {
        comentarios = "%".into();
    }

    ToChange(into_template(
        &paquetes_1,
        &paquetes_2,
        &tikz_libraries,
        &pgfplotsets,
        &temas,
        &fuente,
        &&comentarios,
        &&id,
        &problem,
        &solution,
    ))
}

fn is_template(input: &str, data: &mut Data) -> bool {
    if input.contains("%%% PLANTILLA PARA SUBIR EJERCICIOS A LA BASE DE DATOS DEL PIM") {
        find_year(input, data);
        true
    } else {
        false
    }
}
