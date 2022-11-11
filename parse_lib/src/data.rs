use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Debug,
    fs,
    hash::BuildHasher,
    io::{self, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{
    files::ParseOneError,
    html::{_POSTAMBLE, _PREAMBLE},
    table_friendly::TableFriendly,
    Fields,
};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    fn from_read(
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
    ) -> Result<Self, ParseOneError> {
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
        Ok(Self {
            id: id.parse().map_err(|err| {
                ParseOneError::IMessedUp(format!(
                    "No se pudo interpretar {id} como un número\n{err}"
                ))
            })?,
            temas,
            dificultad,
            fuente: descripcion,
            comentarios,
            historial,
            curso: None,
            enunciado: String::new(),
            paquetes: Vec::new(),
        })
    }

    #[must_use]
    pub fn has_more_data_than(&self, other: &Self) -> Option<(String, String)> {
        for f in Fields::ALL {
            let info_1 = f.get_string(self);
            let info_2 = f.get_string(other);
            if info_1 != info_2 {
                if [String::from("255"), String::new()].contains(&info_2.clone().into_owned()) {
                    continue;
                }

                return Some((info_1.into_owned(), info_2.into_owned()));
            }
        }
        None
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// both entries have non empty data in a field
    pub fn merge_with(&mut self, other: &Self) -> Result<(), String> {
        use crate::FieldContents::{Difficulty, Id, Optional, Str, VecStr};
        for field in Fields::ALL {
            let data_1 = field.get(self);
            let data_2 = field.get(other);
            if data_1 != data_2 {
                let data_1 = data_1.to_owned();
                let data_2 = data_2.to_owned();
                match (data_1, data_2) {
                    (Id(_), Id(_)) => return Err(String::from("Los ids son diferentes")),
                    (VecStr(vec_1), VecStr(vec_2)) => {
                        if vec_1.is_empty() {
                            field.set(self, VecStr(vec_2));
                        } else if !vec_2.is_empty() {
                            return Err(format!(
                                "No se pueden combinar {field}:\n{vec_1:?}\n{vec_2:?}"
                            ));
                        }
                    }
                    (Str(str_1), Str(str_2)) => {
                        if field == Fields::Source {
                            println!("Comparar {str_1} y {str_2}");
                        }
                        if str_1.is_empty() {
                            println!("Reemplazar");
                            field.set(self, Str(str_2));
                            println!("{self:?}");
                        } else if !str_2.is_empty() {
                            return Err(format!(
                                "No se pueden combinar {field}:\n{str_1:?}\n{str_2:?}"
                            ));
                        }
                    }
                    (Difficulty(d1), Difficulty(d2)) => {
                        if d1 == u8::MAX {
                            field.set(self, Difficulty(d2));
                        } else if d2 != u8::MAX {
                            return Err(format!("No se pueden combinar {field}:\n{d1:?}\n{d2:?}"));
                        }
                    }
                    (Optional(o1), Optional(o2)) => {
                        if o1.is_none() {
                            field.set(self, Optional(o2));
                        } else if o2.is_some() {
                            return Err(format!("No se pueden combinar {field}:\n{o1:?}\n{o2:?}"));
                        }
                    }
                    (_, _) => unreachable!("Datos incompatibles"),
                }
            }
        }
        Ok(())
    }

    pub fn sort_packages(&mut self) {
        self.paquetes.sort_by(|x, y| {
            let pgfplotset = (x.contains("pgfplotsset"), y.contains("pgfplotsset"));
            match pgfplotset {
                (true, true) => return x.cmp(y),
                (true, false) => return Ordering::Greater,
                (false, true) => return Ordering::Less,
                (false, false) => {}
            }
            let tikzlibrary = (x.contains("usetikzlibrary"), y.contains("usetikzlibrary"));
            match tikzlibrary {
                (true, true) | (false, false) => x.cmp(y),
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
            }
        });

        self.paquetes.dedup();

        if let Some(i) =
            self.paquetes
                .iter()
                .enumerate()
                .find_map(|(i, x)| if x.is_empty() { Some(i) } else { None })
        {
            self.paquetes.remove(i);
        }
    }
}

#[must_use]
pub fn read_csv(path: &Path) -> (HashMap<usize, Data>, Vec<ParseOneError>) {
    let mut output: HashMap<usize, _> = HashMap::new();
    let mut reader = csv::Reader::from_path(path).expect("Can't open file?");
    let mut errors = vec![];
    for result in reader.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: Read = result.expect("Record is wrong");
        let record = Data::from_read(record);
        match record {
            Ok(record) => {
                output.insert(record.id, record);
            }
            Err(err) => errors.push(err),
        }
    }
    (output, errors)
}

/// .
///
/// # Errors
///
/// This function will return an error if
/// there is a problem serializing
/// or if there is a problem writing the file
pub fn write_json<P: AsRef<Path>, T: BuildHasher>(
    path: P,
    data: &HashMap<usize, Data, T>,
) -> io::Result<()> {
    let string = serde_json::to_string(data)?;
    fs::write(path, string)
}

/// .
///
/// # Errors
///
/// This function will return an error if
///     The file can't be read
///     Json can't be deserialized
// pub fn read_json<P: AsRef<Path>>(json_path: P) -> Result<HashMap<usize, Data>, Error> {
//     let string = fs::read_to_string(&json_path)?;

//     let json = serde_json::from_str(&string)?;

//     println!("Fetched data: {json:#?}");

//     Ok(json)
// }

/// .
///
/// # Errors
///
/// This function will return an error if
/// the file can't be opened
pub fn get_json_string<P: AsRef<Path>>(json_path: P) -> Result<String, Error> {
    let string = fs::read_to_string(&json_path)?;

    Ok(string)
}

pub fn write_csv<P: AsRef<Path>>(data: &[TableFriendly], path: P) {
    let mut writer = csv::Writer::from_path(path).expect("Couldn't create writer");
    for record in data {
        writer.serialize(record).expect("failed to serialize");
    }
    writer.flush().expect("Failed ot flush, ew");
}

pub fn _write_html<T: BuildHasher>(data: &HashMap<usize, Data, T>) {
    let mut writer = fs::File::create("Datos.html").expect("Can't create file"); // let mut writer = csv::Writer::from_path("Datos-out.csv").expect("Can't create file");
    writer
        .write_all(_PREAMBLE.as_bytes())
        .expect("Couldn't start writing");
    let mut data_vec: Vec<_> = data.iter().map(|(_, value)| value.clone()).collect();
    data_vec.sort_by(|d1, d2| d1.id.cmp(&d2.id));
    data_vec
        .into_iter()
        .for_each(|data| _write_one_entry(&data, &mut writer));
    writer
        .write_all(_POSTAMBLE.as_bytes())
        .expect("Couldn't write the end");
    writer.flush().expect("Couldn't flush. Yuck!");
}

fn _write_one_entry<W: io::Write>(data: &Data, writer: &mut W) {
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

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Self::IO(val)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::Serde(val)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => write!(f, "Error reading file: {err}"),
            Self::Serde(err) => write!(f, "Error parsing JSON: {err}"),
        }
    }
}
