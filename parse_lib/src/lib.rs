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

use encoding_rs::mem::convert_latin1_to_utf8;
use process_tex::find_year;
use regex::Regex;
use serde::{Deserialize, Serialize};

mod preamble;
pub mod process_tex;
pub mod run_latex;
/// .
///
/// # Panics
///
/// Panics if I mess up the implementation
///
/// # Errors
///
/// This function will return an error if
/// * The file can't be opened
/// * There's an error reading the file
pub fn parse_file<T: Clone + Debug + AsRef<Path>>(path: T) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = vec![];

    file.read_to_end(&mut buf)?;

    let try_read = String::from_utf8(buf.clone());

    if let Ok(string) = try_read {
        return Ok(string);
    }

    let mut converted_buffer = vec![0; buf.len() * 2];

    let written = convert_latin1_to_utf8(&buf, &mut converted_buffer);

    converted_buffer.truncate(written);

    Ok(String::from_utf8(converted_buffer).expect("Ni después de convertir es utf 8"))
}

/// .
///
/// # Errors
///
/// This function will return an error if there's an io problem
///
/// # Panics
///
/// If I mess up
pub fn parse_all_files<T: BuildHasher>(data: &mut HashMap<usize, Data, T>) -> io::Result<()> {
    let entries = fs::read_dir("ejercicios-in")?;

    for file in entries {
        let file = file?;
        let name = file.file_name();
        let name = name.to_string_lossy();

        if file.file_type()?.is_file() && name.ends_with(".tex") {
            let id: Result<usize, _> = name.split(".tex").next().unwrap().parse();

            let id = if let Ok(id) = id {
                id
            } else {
                println!("Nombre de archivo {name}");
                continue;
            };
            let path = file.path();
            let in_string = parse_file(path).expect("Had problems parsing file");
            let out_path = format!("ejercicios-out/{name}");
            let mut placeholder = Data::new(id);
            let mut to_insert = false;
            let problem_info = data.get_mut(&id).unwrap_or_else(|| {
                println!("{id} no está en la base de datos");
                to_insert = true;
                &mut placeholder
            });
            let out_string = process_tex(&in_string, problem_info);
            fs::write(out_path, out_string)?;
            if to_insert {
                data.insert(id, placeholder);
            }
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadData {
    id: String,
    fecha: String,
    tema1: String,
    tema2: String,
    tema3: String,
    tema4: String,
    dificultad1: String,
    dificultad2: String,
    dificultad3: String,
    descripcion: String,
    historial: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    id: usize,
    temas: Vec<String>,
    dificultad: u8,
    fuente: String,
    historial: Vec<String>,
    comentarios: Vec<String>,
    curso: Option<String>,
    enunciado: String,
}
impl Data {
    const fn new(id: usize) -> Self {
        Self {
            id,
            temas: vec![],
            dificultad: u8::MAX,
            fuente: String::new(),
            historial: vec![],
            comentarios: vec![],
            curso: None,
            enunciado: String::new(),
        }
    }
}

impl Data {
    fn from_read(
        ReadData {
            id,
            fecha: _,
            tema1,
            tema2,
            tema3,
            tema4,
            dificultad1,
            dificultad2,
            dificultad3,
            descripcion,
            historial,
        }: ReadData,
    ) -> Self {
        let temas = [tema1, tema2, tema3, tema4]
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect();
        let mut comentarios: Vec<String> = vec![];
        let dificultades = [dificultad1, dificultad2, dificultad3]
            .into_iter()
            .filter(|x| !x.is_empty())
            .filter_map(|d| {
                let numero: Result<u8, _> = d.parse();
                numero.map_or_else(
                    |_| {
                        comentarios.push(format!("Dificultad: {d}"));
                        None
                    },
                    Some,
                )
            })
            .collect::<Vec<_>>();
        let dificultad = dificultades.first().copied().unwrap_or(u8::MAX);
        let historial = if historial.is_empty() {
            vec![]
        } else {
            vec![historial]
        };
        Self {
            id: id.parse().unwrap(),
            temas,
            dificultad,
            fuente: descripcion,
            comentarios,
            historial,
            curso: None,
            enunciado: String::new(),
        }
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if there's an i/o error
pub fn read_csv() -> io::Result<HashMap<usize, Data>> {
    let mut output: HashMap<usize, _> = HashMap::new();
    let mut reader = csv::Reader::from_path("Datos.csv")?;
    for result in reader.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: ReadData = result?;
        let record = Data::from_read(record);
        output.insert(record.id, record);
    }
    Ok(output)
}

/// .
///
/// # Errors
///
/// This function will return an error if there are io problems
pub fn write_json<T: BuildHasher>(data: &HashMap<usize, Data, T>) -> io::Result<()> {
    let string = serde_json::to_string(data)?;
    fs::write("data.json", string)
}

/// .
///
/// # Panics
///
/// Panics if I fuck up.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn process_tex(input: &str, data: &mut Data) -> String {
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
