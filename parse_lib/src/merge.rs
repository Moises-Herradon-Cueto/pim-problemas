use crate::files::ParseOneInfo;
use crate::parsing::{self, find_info_from_template};
use crate::{data::Data, files::ParseOneError};

use crate::preamble::into_template;

use ParseResult::{Template, ToChange};

pub enum ParseResult {
    Template(Vec<(usize, ParseOneInfo)>),
    ToChange(String),
}

pub fn string_and_data(input: &str, data: &mut Data) -> Result<ParseResult, ParseOneError> {
    let problem = parsing::problem(data.id, input)?;

    data.enunciado = problem.to_owned();

    if let Some((data_in_tex, mut errors)) = is_template(input, data)? {
        let more_errors = data
            .merge_with(&data_in_tex)
            .into_iter()
            .map(|x| (data.id, x));
        errors.extend(more_errors);
        return Ok(Template(errors));
    }

    let solution = parsing::solution(data.id, input)?;

    parsing::packages(data, input)?;

    data.sort_packages();

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

    let dificultad = if data.dificultad == u8::MAX {
        "%".into()
    } else {
        data.dificultad.to_string()
    };

    Ok(ToChange(into_template(
        &data.paquetes.join("\n"),
        &temas,
        &dificultad,
        &fuente,
        &&comentarios,
        &&id,
        &problem,
        &solution,
    )))
}

fn is_template(
    input: &str,
    data: &mut Data,
) -> Result<Option<(Data, Vec<(usize, ParseOneInfo)>)>, ParseOneError> {
    if input.contains("%%% PLANTILLA PARA SUBIR EJERCICIOS A LA BASE DE DATOS DEL PIM") {
        let (data_in_template, missing_fields) = find_info_from_template(input)?;
        let missing_fields = missing_fields
            .into_iter()
            .map(|f| (data_in_template.id, ParseOneInfo::MissingInTex(f)))
            .collect();
        Ok(Some((data_in_template, missing_fields)))
    } else {
        Ok(None)
    }
}
