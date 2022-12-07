use crate::files::ParseOneInfo;
use crate::parsing::{self, find_info_from_template};
use crate::Fields;
use crate::{data::Data, files::ParseOneError};

use crate::preamble::into_template;

use ParseResult::{Template, ToChange};

pub enum ParseResult {
    Template,
    ToChange {
        content: String,
        error: Vec<(usize, ParseOneInfo)>,
        is_in_template: bool,
    },
}

pub fn string_and_data(input: &str, data: &mut Data) -> Result<ParseResult, ParseOneError> {
    if data.enunciado.is_empty() {
        let problem = parsing::problem(data.id, input)?;
        data.enunciado = problem.trim().to_owned();
    }

    let mut errors = vec![];

    let mut is_in_template = false;

    if let Some((data_in_tex, _)) = is_template(data.id, input)? {
        let more_errors = data.merge_with(&data_in_tex);
        // if data.id == 2200035 {
        //     println!("{errors:#?}");
        //     println!("{more_errors:#?}");
        // }
        if more_errors.is_empty() {
            return Ok(Template);
        }
        let more_errors = more_errors.into_iter().map(|x| (data.id, x));
        errors.extend(more_errors);
        is_in_template = true;
    }

    parsing::packages(data, input)?;

    data.sort_packages();

    let missing_fields: Vec<Fields> = Fields::ALL
        .into_iter()
        .filter(|field| !field.is_optional() && field.get(data).is_empty())
        .collect();

    if !missing_fields.is_empty() {
        errors.push((data.id, ParseOneInfo::MissingInDb(missing_fields)));
    };

    Ok(ToChange {
        content: overwrite_document_data(input, data)?,
        error: errors,
        is_in_template,
    })
}

fn is_template(
    id: usize,
    input: &str,
) -> Result<Option<(Data, Option<ParseOneInfo>)>, ParseOneError> {
    if input.contains("%%% PLANTILLA PARA SUBIR EJERCICIOS A LA BASE DE DATOS DEL PIM") {
        let (data_in_template, missing_fields) = find_info_from_template(id, input)?;
        if missing_fields.is_empty() {
            Ok(Some((data_in_template, None)))
        } else {
            Ok(Some((
                data_in_template,
                Some(ParseOneInfo::MissingInTex(missing_fields)),
            )))
        }
    } else {
        Ok(None)
    }
}

pub fn overwrite_document_data(input: &str, data: &Data) -> Result<String, ParseOneError> {
    let document = parsing::document(data.id, input)?;

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
    let mut comentarios = data.comentarios.as_str();

    if comentarios.is_empty() {
        comentarios = "%";
    }

    let mut curso = data.curso.as_str();
    if curso.is_empty() {
        curso = "%";
    }

    let mut historial = data.historial.as_str();

    if historial.is_empty() {
        historial = "%";
    }

    let dificultad = if data.dificultad == u8::MAX {
        "%".into()
    } else {
        data.dificultad.to_string()
    };

    Ok(into_template(
        &data.paquetes,
        &temas,
        &dificultad,
        &historial,
        &curso,
        &fuente,
        &comentarios,
        &id,
        &document,
    ))
}
