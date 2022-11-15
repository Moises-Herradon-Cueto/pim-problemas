use serde::{Deserialize, Serialize};

use crate::Data;

#[derive(Serialize, Deserialize)]
pub struct TableFriendly {
    pub id: usize,
    pub temas: String,
    pub dificultad: u8,
    pub fuente: String,
    pub historial: String,
    pub comentarios: String,
    pub curso: Option<String>,
    pub enunciado: String,
    pub paquetes: String,
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
        }: Data,
    ) -> Self {
        Self {
            id,
            temas: temas.join(","),
            dificultad,
            fuente,
            historial: historial.join(","),
            comentarios: comentarios.join("\n"),
            curso,
            enunciado,
            paquetes: paquetes.join("\n"),
        }
    }
}
