use crate::parsing;
use crate::{data::Data, files::ParseOneError};

use crate::preamble::into_template;
use crate::process_tex::find_year;

use ParseResult::{Template, ToChange};

pub enum ParseResult {
    Template,
    ToChange(String),
}

pub fn string_and_data(input: &str, data: &mut Data) -> Result<ParseResult, ParseOneError> {
    let problem = parsing::problem(data.id, input)?;

    data.enunciado = problem.to_owned();

    if is_template(input, data)? {
        return Ok(Template);
    }

    let solution = parsing::solution(data.id, input)?;

    parsing::packages(data, input)?;

    let mut temas = data.temas.join(", ");

    if temas.is_empty() {
        temas = "%".into();
    }

    let id = data.id;

    let mut fuente = &data.fuente;
    let percent = String::from("%");

    if fuente.is_empty() {
        fuente = &percent;
    }
    let mut comentarios = data.comentarios.join(", ");

    if comentarios.is_empty() {
        comentarios = "%".into();
    }

    Ok(ToChange(into_template(
        &data.paquetes.join("\n"),
        &temas,
        &fuente,
        &&comentarios,
        &&id,
        &problem,
        &solution,
    )))
}

fn is_template(input: &str, data: &mut Data) -> Result<bool, ParseOneError> {
    if input.contains("%%% PLANTILLA PARA SUBIR EJERCICIOS A LA BASE DE DATOS DEL PIM") {
        find_year(input, data)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
