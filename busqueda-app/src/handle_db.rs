use pim_lib::Data;
use serde::{Deserialize, Serialize};

type Db = Vec<Data>;

pub enum Error {}

#[derive(Serialize, Deserialize)]
pub struct FetchedData {
    #[serde(rename = "ID")]
    id: usize,
    #[serde(rename = "ID_Autor")]
    id_autor: String,
    #[serde(rename = "Titulo")]
    titulo: String,
    #[serde(rename = "Dificultad")]
    dificultad: Option<u8>,
    #[serde(rename = "Curso")]
    curso: Option<String>,
    #[serde(rename = "Procedencia")]
    procedencia: String,
    #[serde(rename = "Preambulo")]
    preambulo: String,
    #[serde(rename = "Descripcion")]
    descripcion: String,
    #[serde(rename = "TEX_URL")]
    url: String,
    #[serde(rename = "Comentarios")]
    comentarios: String,
    #[serde(rename = "Temas")]
    temas: Option<String>,
    #[serde(rename = "Hojas")]
    hojas: Option<String>,
}

impl From<FetchedData> for Data {
    fn from(
        FetchedData {
            id,
            id_autor,
            titulo,
            dificultad,
            curso,
            procedencia,
            preambulo,
            descripcion,
            url,
            comentarios,
            temas,
            hojas,
        }: FetchedData,
    ) -> Self {
        Self {
            id,
            temas: temas.map_or(Vec::new(), |temas| {
                temas.split(',').map(ToOwned::to_owned).collect()
            }),
            titulo,
            dificultad: dificultad.unwrap_or(u8::MAX),
            fuente: procedencia,
            historial: hojas.unwrap_or_default(),
            comentarios,
            curso: curso.unwrap_or_default(),
            enunciado: descripcion,
            paquetes: preambulo,
            url,
            id_autor,
        }
    }
}
// impl From<FetchedData> for Data {
//     type Error = std::num::ParseIntError;

//     fn try_from(
//         FetchedProblem {
//             id,
//             id_autor,
//             titulo,
//             dificultad,
//             curso,
//             fuente,
//             paquetes,
//             enunciado,
//             url,
//             comentarios,
//         }: FetchedProblem,
//     ) -> Result<Self, Self::Error> {
//         Ok(Data {
//             id: id.parse()?,
//             temas: Vec::new(),
//             dificultad: dificultad.parse()?,
//             fuente,
//             historial: Vec::new(),
//             comentarios: comentarios.split("\n").map(ToOwned::to_owned).collect(),
//             curso: Some(curso),
//             enunciado,
//             paquetes: paquetes.split('\n').map(ToOwned::to_owned).collect(),
//             id_autor,
//             url,
//         })
//     }
// }
