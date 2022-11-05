use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    hash::BuildHasher,
    io::{self, Write},
};

use serde::{Deserialize, Serialize};

use crate::html::{POSTAMBLE, PREAMBLE};

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
    pub paquetes: Vec<String>,
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
            paquetes: Vec::new(),
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
            paquetes: Vec::new(),
        }
    }
}

#[must_use]
pub fn read_csv() -> HashMap<usize, Data> {
    let mut output: HashMap<usize, _> = HashMap::new();
    let mut reader = csv::Reader::from_path("Datos.csv").expect("Can't open file?");
    for result in reader.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: Read = result.expect("Record is wrong");
        let record = Data::from_read(record);
        output.insert(record.id, record);
    }
    output
}

pub fn write_json<T: BuildHasher>(data: &HashMap<usize, Data, T>) -> io::Result<()> {
    let string = serde_json::to_string(data)?;
    fs::write("data.json", string)
}

#[must_use]
pub fn read_json() -> HashMap<usize, Data> {
    let string = fs::read_to_string("data.json").expect("Couldn't read data");

    serde_json::from_str(&string).expect("Failed to deserialize")
}

pub fn write_html<T: BuildHasher>(data: &HashMap<usize, Data, T>) {
    let mut writer = fs::File::create("Datos.html").expect("Can't create file"); // let mut writer = csv::Writer::from_path("Datos-out.csv").expect("Can't create file");
    writer
        .write_all(PREAMBLE.as_bytes())
        .expect("Couldn't start writing");
    let mut data_vec: Vec<_> = data.iter().map(|(_, value)| value.clone()).collect();
    data_vec.sort_by(|d1, d2| d1.id.cmp(&d2.id));
    data_vec
        .into_iter()
        .for_each(|data| write_one_entry(&data, &mut writer));
    writer
        .write_all(POSTAMBLE.as_bytes())
        .expect("Couldn't write the end");
    writer.flush().expect("Couldn't flush. Yuck!");
}

fn write_one_entry<W: io::Write>(data: &Data, writer: &mut W) {
    let to_write = format!(
        "
        <tr>
        <td>
        {}
        </td>
        <td>
        {}
        </td>
        <td>
        {}
        </td>
        <td>
        {}
        </td>
        <td>
        {}
        </td>
        <td>
        {}
        </td>
        <td>
        {}
        </td>
        <td>
        {}
        </td>
        </tr>
        ",
        data.id,
        data.temas.join("<br/>"),
        data.dificultad,
        data.fuente,
        data.historial.join("<br/>"),
        data.comentarios.join("<br/>"),
        data.curso.as_ref().unwrap_or(&String::from("Vacío")),
        data.enunciado
    );

    writer
        .write_all(to_write.as_bytes())
        .expect("Couldn't write entry");
}
