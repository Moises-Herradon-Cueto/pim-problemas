use pim_lib::{Curso, Data};
use serde::{Deserialize, Serialize};

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
    curso: Option<Curso>,
    #[serde(rename = "Procedencia")]
    procedencia: String,
    #[serde(rename = "Preambulo")]
    preambulo: String,
    #[serde(rename = "Descripcion")]
    descripcion: String,
    #[serde(rename = "TEX_URL")]
    tex_url: String,
    #[serde(rename = "PDF_URL")]
    pdf_url: String,
    #[serde(rename = "Comentarios")]
    comentarios: Option<String>,
    #[serde(rename = "Temas")]
    temas: Option<String>,
    #[serde(rename = "Hojas")]
    hojas: Option<String>,
    #[serde(rename = "Figuras")]
    figuras: Option<String>,
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
            tex_url,
            pdf_url,
            comentarios,
            temas,
            hojas,
            figuras,
        }: FetchedData,
    ) -> Self {
        Self {
            id,
            temas: temas.map_or(Vec::new(), |temas| {
                temas.split(',').map(ToOwned::to_owned).collect()
            }),
            figuras: figuras.map_or(Vec::new(), |figuras| {
                figuras.split(',').map(ToOwned::to_owned).collect()
            }),
            titulo,
            dificultad: dificultad.unwrap_or(u8::MAX),
            fuente: procedencia,
            historial: hojas.unwrap_or_default(),
            comentarios: comentarios.unwrap_or_default(),
            curso,
            enunciado: descripcion,
            paquetes: preambulo,
            tex_url,
            pdf_url,
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
