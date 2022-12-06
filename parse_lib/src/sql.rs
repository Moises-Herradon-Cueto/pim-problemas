use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::Data;

pub fn into<T, K>(database: &HashMap<T, Data, K>) -> String {
    let mut vec: Vec<_> = database.values().collect();
    vec.sort_by(|p1, p2| p1.id.cmp(&p2.id));

    let problems_columns = vec.iter().map(|data| get_columns(data));
    let problems_columns = join_with_comma_newline(problems_columns);

    let pim_temas: HashSet<_> = vec
        .iter()
        .flat_map(|d| d.temas.iter().map(|tema| process_tema(tema)))
        .collect();
    let pim_temas = pim_temas.into_iter().map(|tema| format!("('{tema}')"));
    let pim_temas = join_with_comma_newline(pim_temas);

    let topics_columns: String = vec
        .iter()
        .flat_map(|data| data.temas.iter().map(|tema| (data.id, process_tema(tema))))
        .map(|(id, tema)| {
            format!(
                "INSERT INTO pim_problemas_temas (
ID_Problema, Tema   
) SELECT ID, '{tema}'
FROM pim_problemas WHERE
Titulo = '{id}';\n"
            )
        })
        .collect();

    let sheets_columns: HashSet<_> = vec
        .iter()
        .flat_map(|data| data.historial.split(','))
        .map(|sheet| format!("('{}', 2022)", sheet.trim()))
        .collect();
    let sheets_columns = join_with_comma_newline(sheets_columns.into_iter());

    let sheets_problems_columns: String = vec
        .iter()
        .flat_map(|data| data.historial.split(',').map(|sheet| (data.id, sheet)))
        .map(|(id, sheet)| {
            format!(
                "
INSERT INTO pim_problemas_hojas (ID_Problema, ID_Hoja)
SELECT pim_problemas.ID,hojas.ID
FROM pim_problemas
    CROSS JOIN (
        SELECT ID
        FROM pim_hojas
        WHERE Titulo = '{sheet}'
    ) as hojas
    WHERE pim_problemas.titulo = '{id}';\n"
            )
        })
        .collect();

    format!(
        "INSERT INTO pim_problemas (
ID_Autor,
Titulo,
Dificultad,
Curso,
Procedencia,
Preambulo,
Descripcion,
TEX_URL,
Comentarios
) VALUES {problems_columns};

INSERT INTO pim_temas (
    Titulo
) VALUES {pim_temas};

{topics_columns}

INSERT INTO pim_hojas (Titulo, Curso) VALUES
{sheets_columns};

{sheets_problems_columns}"
    )
}

// ID_Autor VARCHAR(255) NOT NULL,
// Titulo VARCHAR(255) NOT NULL,
// Dificultad TINYINT NOT NULL,
// CONSTRAINT Dificultad1a10 CHECK (Dificultad >= 1 && Dificultad <= 10),
// Curso ENUM(
//     '1 Primaria',
//     '2 Primaria',
//     '3 Primaria',
//     '4 Primaria',
//     '5 Primaria',
//     '6 Primaria',
//     '1 ESO',
//     '2 ESO',
//     '3 ESO',
//     '4 ESO',
//     '1 BACH',
//     '2 BACH'
// ) NOT NULL,
// Procedencia VARCHAR(255) CHARACTER SET utf8,
// Preambulo TEXT CHARACTER SET utf8,
// Descripcion TEXT CHARACTER SET utf8,
// TEX_URL VARCHAR(255),
// Comentarios TEXT CHARACTER SET utf8,

fn get_columns(data: &Data) -> String {
    let id_autor = "Antiguo";
    let titulo = data.id;
    let dificultad = if data.dificultad == u8::MAX {
        String::from("NULL")
    } else {
        data.dificultad.to_string()
    };
    let curso_with_commas = curso(&data.curso);
    let fuente = escape(&data.fuente);
    let paquetes = escape(&data.paquetes);
    let enunciado = escape(&data.enunciado);
    let comentarios = escape(&data.comentarios);
    format!("('{id_autor}', '{titulo}', {dificultad}, {curso_with_commas}, '{fuente}', '{paquetes}', '{enunciado}','/PIM/externos/intranet/files/{}.tex', '{comentarios}')", data.id)
}

fn join_with_comma_newline<S: Display, T: Iterator<Item = S>>(mut iter: T) -> String {
    let Some(first) = iter.next() else {
        return String::new();
    };
    iter.fold(first.to_string(), |acc, next| format!("{acc},\n{next}"))
}

fn escape(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

fn curso(input: &str) -> String {
    if input.is_empty() {
        String::from("NULL")
    } else {
        match input {
            "1ESO" => String::from("'1 ESO'"),
            "2ESO" => String::from("'2 ESO'"),
            "3ESO" => String::from("'3 ESO'"),
            "4ESO" => String::from("'4 ESO'"),
            "1Primaria" => String::from("'1 Primaria'"),
            "2Primaria" => String::from("'2 Primaria'"),
            "3Primaria" => String::from("'3 Primaria'"),
            "4Primaria" => String::from("'4 Primaria'"),
            "5Primaria" => String::from("'5 Primaria'"),
            "6Primaria" => String::from("'6 Primaria'"),
            "1BACH" => String::from("'1 BACH'"),
            "2BACH" => String::from("'2 BACH'"),
            x => panic!("Curso: {x}"),
        }
    }
}

fn process_tema(topic: &str) -> String {
    let topic = topic.to_lowercase();
    match topic.as_str() {
        "planimetria" | "planemetria" | "planemetría" => "planimetría".into(),
        "angulos" => "ángulos".into(),
        _ => topic,
    }
}
