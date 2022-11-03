#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::{
    collections::HashMap,
    fs,
    hash::BuildHasher,
    io::{self, Read},
    path::Path,
};

use encoding_rs::mem::convert_latin1_to_utf8;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};

mod preamble;
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
pub fn parse_file<T: AsRef<Path>>(path: T) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = vec![];

    file.read_to_end(&mut buf)?;

    let try_read = String::from_utf8(buf.clone());

    match try_read {
        Ok(string) => return Ok(string),
        Err(err) => {
            log::info!("Not utf8:{err:?}");
        }
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
pub fn parse_all_files<T: BuildHasher>(data: &HashMap<usize, Data, T>) -> io::Result<()> {
    let entries = fs::read_dir("ejercicios-in")?;

    for file in entries {
        let file = file?;
        let name = file.file_name();
        let name = name.to_string_lossy();
        let id: Result<usize, _> = name.split(".tex").next().unwrap().parse();

        let id = if let Ok(id) = id {
            id
        } else {
            log::warn!("Nombre de archivo {name}");
            continue;
        };

        if file.file_type()?.is_file() && name.ends_with(".tex") {
            let path = file.path();
            let in_string = parse_file(path).expect("Had problems parsing file");
            let out_path = format!("ejercicios-out/{name}");
            let placeholder = Data::new(id);
            let problem_info = data.get(&id).unwrap_or_else(|| {
                log::warn!("El problema {id} no está en la base de datos");
                &placeholder
            });
            let out_string = process_tex(&in_string, problem_info);
            fs::write(out_path, out_string)?;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    id: usize,
    temas: Vec<String>,
    dificultad: u8,
    fuente: String,
    historial: Vec<String>,
    comentarios: Vec<String>,
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
pub fn process_tex(input: &str, data: &Data) -> String {
    let problem_regex = Regex::new(r"(?s)\\begin\{ejer\}(.*)\\end\{ejer\}").expect("regex wrong");
    let problem = problem_regex
        .captures_iter(input)
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let problem_regex = Regex::new(r"(?s)\\begin\{ejer\}(.*)\\end\{ejer\}").expect("regex wrong");
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

%%% Fin de paquetes extra


% Introduce los temas separados por comas
% Por ejemplo
% \\temas{{
% Inducción, Numeritos
% }}
\\temas{{
%
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
%
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
%
}}

\\begin{{document}}

\\datos



 
\\begin{{ejer}}
{problem}
\\end{{ejer}}


 

 
 
\\begin{{proof}}[Solución]
Claro que las hay: $3^2+4^2=5^2$.
\\end{{proof}}

\\end{{document}}

    
    "
    )
}
