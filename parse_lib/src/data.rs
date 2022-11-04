use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    hash::BuildHasher,
    io::{self},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Read {
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
    pub id: usize,
    pub temas: Vec<String>,
    pub dificultad: u8,
    pub fuente: String,
    pub historial: Vec<String>,
    pub comentarios: Vec<String>,
    pub curso: Option<String>,
    pub enunciado: String,
}
impl Data {
    #[must_use]
    pub const fn new(id: usize) -> Self {
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
    /// .
    ///
    /// # Panics
    ///
    /// Panics if I mess up
    #[must_use]
    pub fn from_read(
        Read {
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
        }: Read,
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
        let record: Read = result?;
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
