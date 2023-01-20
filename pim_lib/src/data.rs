use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Debug,
    fs,
    hash::BuildHasher,
    io::{self, Write},
    path::Path,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    files::{ParseOneError, ParseOneInfo},
    html::{POSTAMBLE, PREAMBLE},
    table_friendly::TableFriendly,
    FieldContents, Fields,
};
use Curso::{
    Bach1, Bach2, Primaria1, Primaria2, Primaria3, Primaria4, Primaria5, Primaria6, ESO1, ESO2,
    ESO3, ESO4,
};

pub mod enunciado;
pub mod packages;

pub const CURSOS: [Curso; 12] = [
    Primaria1, Primaria2, Primaria3, Primaria4, Primaria5, Primaria6, ESO1, ESO2, ESO3, ESO4,
    Bach1, Bach2,
];

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Curso {
    #[serde(rename = "1 Primaria")]
    Primaria1 = 0,
    #[serde(rename = "2 Primaria")]
    Primaria2 = 1,
    #[serde(rename = "3 Primaria")]
    Primaria3 = 2,
    #[serde(rename = "4 Primaria")]
    Primaria4 = 3,
    #[serde(rename = "5 Primaria")]
    Primaria5 = 4,
    #[serde(rename = "6 Primaria")]
    Primaria6 = 5,
    #[serde(rename = "1 ESO")]
    ESO1 = 6,
    #[serde(rename = "2 ESO")]
    ESO2 = 7,
    #[serde(rename = "3 ESO")]
    ESO3 = 8,
    #[serde(rename = "4 ESO")]
    ESO4 = 9,
    #[serde(rename = "1 BACH")]
    Bach1 = 10,
    #[serde(rename = "2 BACH")]
    Bach2 = 11,
}

impl FromStr for Curso {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace(' ', "").as_str() {
            "1primaria" => Ok(Primaria1),
            "2primaria" => Ok(Primaria2),
            "3primaria" => Ok(Primaria3),
            "4primaria" => Ok(Primaria4),
            "5primaria" => Ok(Primaria5),
            "6primaria" => Ok(Primaria6),
            "1eso" => Ok(ESO1),
            "2eso" => Ok(ESO2),
            "3eso" => Ok(ESO3),
            "4eso" => Ok(ESO4),
            "1bach" => Ok(Bach1),
            "2bach" => Ok(Bach2),
            _ => Err(s.to_owned()),
        }
    }
}

impl std::fmt::Display for Curso {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primaria1 => write!(f, "1Primaria"),
            Primaria2 => write!(f, "2Primaria"),
            Primaria3 => write!(f, "3Primaria"),
            Primaria4 => write!(f, "4Primaria"),
            Primaria5 => write!(f, "5Primaria"),
            Primaria6 => write!(f, "6Primaria"),
            ESO1 => write!(f, "1ESO"),
            ESO2 => write!(f, "2ESO"),
            ESO3 => write!(f, "3ESO"),
            ESO4 => write!(f, "4ESO"),
            Bach1 => write!(f, "1Bach"),
            Bach2 => write!(f, "2Bach"),
        }
    }
}

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
    pub titulo: String,
    pub temas: Vec<String>,
    pub dificultad: u8,
    pub fuente: String,
    pub historial: String,
    pub comentarios: String,
    pub curso: Option<Curso>,
    pub enunciado: String,
    pub paquetes: String,
    pub tex_url: String,
    pub pdf_url: String,
    pub id_autor: String,
    pub figuras: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Old {
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

impl From<Old> for Data {
    fn from(
        Old {
            id,
            temas,
            dificultad,
            fuente,
            historial,
            comentarios,
            curso,
            enunciado,
            paquetes,
        }: Old,
    ) -> Self {
        Self {
            id,
            temas,
            dificultad,
            fuente,
            titulo: id.to_string(),
            historial: historial.join("\n"),
            comentarios: comentarios.join("\n"),
            curso: curso.map(|c| c.parse().ok()).unwrap_or_default(),
            enunciado,
            paquetes: paquetes.join("\n"),
            tex_url: String::new(),
            pdf_url: String::new(),
            id_autor: String::new(),
            figuras: Vec::new(),
        }
    }
}

impl Data {
    #[must_use]
    pub fn new(id: usize) -> Self {
        Self {
            id,
            titulo: id.to_string(),
            temas: vec![],
            dificultad: u8::MAX,
            fuente: String::new(),
            historial: String::new(),
            comentarios: String::new(),
            curso: None,
            enunciado: String::new(),
            paquetes: String::new(),
            tex_url: String::new(),
            pdf_url: String::new(),
            id_autor: String::new(),
            figuras: Vec::new(),
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
        let mut comentarios = String::new();
        let dificultades = [dificultad1, dificultad2, dificultad3]
            .into_iter()
            .filter(|x| !x.is_empty())
            .filter_map(|d| {
                let numero: Result<u8, _> = d.parse();
                numero.map_or_else(
                    |_| {
                        comentarios.push_str(&format!("Dificultad: {d}\n"));
                        None
                    },
                    Some,
                )
            })
            .collect::<Vec<_>>();
        let dificultad = dificultades.first().copied().unwrap_or(u8::MAX);
        Ok(Self {
            id: id.parse().map_err(|err| {
                ParseOneError::IMessedUp(format!(
                    "No se pudo interpretar {id} como un número\n{err}"
                ))
            })?,
            titulo: id,
            temas,
            dificultad,
            fuente: descripcion,
            comentarios,
            historial,
            curso: None,
            enunciado: String::new(),
            paquetes: String::new(),
            id_autor: String::new(),
            tex_url: String::new(),
            pdf_url: String::new(),
            figuras: Vec::new(),
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

    pub fn set(&mut self, content: FieldContents) {
        content.set(self);
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// both entries have non empty data in a field
    pub fn merge_with(&mut self, tex_data: &Self) -> Vec<ParseOneInfo> {
        let mut missing_in_db = vec![];
        let mut missing_in_tex = vec![];
        let mut discrepancies = vec![];
        for field in Fields::ALL {
            if matches!(field, Fields::Problem) {
                continue;
            }
            let data_1 = field.get(self);
            let data_2 = field.get(tex_data);
            if data_1 != data_2 {
                let data_1 = data_1.to_owned();
                let data_2 = data_2.to_owned();
                if data_1.is_empty() {
                    self.set(data_2);
                    missing_in_db.push(field);
                } else if data_2.is_empty() {
                    missing_in_tex.push(field);
                } else {
                    discrepancies.push(ParseOneInfo::Incompatible {
                        db: data_1,
                        tex: data_2,
                    });
                }
            }
        }
        if !missing_in_db.is_empty() {
            discrepancies.push(ParseOneInfo::MissingInDb(missing_in_db));
        }
        if !missing_in_tex.is_empty() {
            discrepancies.push(ParseOneInfo::MissingInTex(missing_in_tex));
        }
        discrepancies
    }

    pub fn sort_packages(&mut self) {
        let mut paquetes: Vec<&str> = self.paquetes.split('\n').collect();
        paquetes.sort_by(|x, y| {
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

        paquetes.dedup();

        if let Some(i) =
            paquetes
                .iter()
                .enumerate()
                .find_map(|(i, x)| if x.is_empty() { Some(i) } else { None })
        {
            paquetes.remove(i);
        }
        self.paquetes = paquetes.join("\n");
    }

    pub fn trim(&mut self) {
        split_vec(&mut self.temas);
        self.historial = self.historial.trim().to_owned();
        self.comentarios = self.comentarios.trim().to_owned();
        self.paquetes = self.paquetes.trim().to_owned();
        self.fuente = self.fuente.trim().to_owned();
    }
}

fn split_vec(vec: &mut Vec<String>) {
    let new_vec = vec
        .iter()
        .flat_map(|x| x.split(',').map(str::trim).filter(|x| !x.is_empty()))
        .map(std::borrow::ToOwned::to_owned)
        .collect();
    *vec = new_vec;
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
    let string = serde_json::to_string_pretty(data)?;
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
pub fn get_json_string<P: AsRef<Path>>(json_path: P) -> Result<String, String> {
    let string = fs::read_to_string(&json_path).map_err(|err| {
        println!("{}", json_path.as_ref().display());
        format!(
            "Error attempting to read {}.\n{err}",
            json_path.as_ref().display()
        )
    })?;

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
        .write_all(PREAMBLE.as_bytes())
        .expect("Couldn't start writing");
    let mut data_vec: Vec<_> = data.iter().map(|(_, value)| value.clone()).collect();
    data_vec.sort_by(|d1, d2| d1.id.cmp(&d2.id));
    data_vec
        .into_iter()
        .for_each(|data| _write_one_entry(&data, &mut writer));
    writer
        .write_all(POSTAMBLE.as_bytes())
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
        data.historial.replace('\n', "<br/>"),
        data.comentarios.replace('\n', "<br/>"),
        data.curso.map_or(String::new(), |c| c.to_string()),
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

#[cfg(test)]
mod tests {
    use crate::Data;

    #[test]
    fn try_serde() {
        let data = Data::new(0);
        let data_json = serde_json::to_string_pretty(&data).unwrap();
        println!("{data_json}");
    }
}
