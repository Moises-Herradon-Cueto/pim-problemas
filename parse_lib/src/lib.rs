#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    hash::BuildHasher,
    io::{self, Read},
    path::Path,
};

use data::Data;
use encoding_rs::mem::convert_latin1_to_utf8;
use process_tex::find_year;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub mod data;
pub mod files;
pub mod pdflatex;
mod preamble;
pub mod process_tex;

/// .
///
/// # Panics
///
/// Panics if I fuck up.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn merge_string_data(input: &str, data: &mut Data) -> String {
    if input.contains("%%% PLANTILLA PARA SUBIR EJERCICIOS A LA BASE DE DATOS DEL PIM") {
        // println!("{} está en la plantilla", data.id);
        find_year(input, data);
        return input.to_owned();
    }
    let problem_regex = Regex::new(r"(?s)\\begin\{ejer\}(.*)\\end\{ejer\}").expect("regex wrong");
    let problem = problem_regex
        .captures_iter(input)
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

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
        .expect("fucked up")
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
        .expect("fucked up")
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
        .expect("fucked up")
        .captures_iter(input)
        .map(|result| {
            let package = result.get(1).unwrap().as_str();
            format!("\\usetikzlibrary{{{package}}}\n")
        })
        .collect();
    let pgfplotsets: String = Regex::new(r"\\pgfplotsset\{(.*)}")
        .expect("fucked up")
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

    format!(
        "
% !TeX encoding = UTF-8



%%% PLANTILLA PARA SUBIR EJERCICIOS A LA BASE DE DATOS DEL PIM
\\documentclass[12pt,a4paper]{{article}}
\\usepackage[utf8]{{inputenc}}
\\usepackage[spanish]{{babel}}
\\usepackage{{pim}}

% Si necesitas más paquetes, añádelos debajo de la siguiente línea
%%% Paquetes extra
{paquetes_1}
{paquetes_2}
{tikz_libraries}
{pgfplotsets}
%%% Fin de paquetes extra


% Introduce los temas separados por comas
% Por ejemplo
% \\temas{{
% Inducción, Numeritos
% }}
\\temas{{
{temas}
}}

% Dificultad del 1 al 10
% \\dificultad{{
% 10
% }}
\\dificultad{{
%
}}

% De dónde viene el problema
% \\fuente{{
% Aritmética de Diofanto, capítulo 1.
% }}
\\fuente{{
{fuente}
}}

% Curso a partir del cual se puede poner el problema
% Opciones:
% 1Primaria, 2Primaria ... 6Primaria
% 1ESO, 2ESO, 3ESO, 4ESO
% 1Bach, 2Bach
% \\curso{{
% 1ESO
% }}
\\curso{{
%
}}

% Descomentar para restringir el acceso:
%\\acceso{{
%Sí
%}}

% Comentarios, separados por comas
% \\comentarios{{
% Un problema muy fácil, les salió a todos
% }}
\\comentarios{{
{comentarios}
}}

\\id{{
{id}
}}

\\begin{{document}}

\\datos



 
\\begin{{ejer}}
{problem}
\\end{{ejer}}


 

 
 
\\begin{{proof}}[Solución]
{solution}
\\end{{proof}}

\\end{{document}}

    
    "
    )
}
