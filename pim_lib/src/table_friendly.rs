use serde::{Deserialize, Serialize};

use crate::Data;

#[derive(Serialize, Deserialize)]
pub struct TableFriendly {
    pub id: usize,
    pub titulo: String,
    pub temas: String,
    pub dificultad: u8,
    pub fuente: String,
    pub historial: String,
    pub comentarios: String,
    pub curso: String,
    pub enunciado: String,
    pub paquetes: String,
    pub url: String,
    pub id_autor: String,
    pub figuras: String,
}

impl From<Data> for TableFriendly {
    fn from(
        Data {
            id,
            temas,
            dificultad,
            fuente,
            historial,
            comentarios,
            curso,
            enunciado,
            paquetes,
            url,
            id_autor,
            titulo,
            figuras,
        }: Data,
    ) -> Self {
        Self {
            id,
            temas: temas.join(","),
            figuras: figuras.join(","),
            dificultad,
            fuente,
            historial,
            comentarios,
            curso,
            enunciado,
            paquetes,
            url,
            id_autor,
            titulo,
        }
    }
}
