use regex::Regex;

use crate::{data::Data, files::ParseOneError, Fields};

pub fn problem(id: usize, input: &str) -> Result<&str, ParseOneError> {
    let problem_regex = Regex::new(r"(?s)\\begin\{ejer\}(.*)\\end\{ejer\}")
        .map_err(|err| ParseOneError::IMessedUp(format!("I messed up making the regex: {err}")))?;
    let output = problem_regex
        .captures_iter(input)
        .next()
        .ok_or(ParseOneError::ProblemNotFound(id))?
        .get(1)
        .ok_or_else(|| {
            ParseOneError::IMessedUp(format!(
                "The captured group should have an entry, parsing problem {id}"
            ))
        })?
        .as_str();
    Ok(output)
}

pub fn solution(id: usize, input: &str) -> Result<&str, ParseOneError> {
    let sol_regex = [
        Regex::new(r"(?s)\\begin\{proof\}\[Solución\](.*)\\end\{proof\}").map_err(|err| {
            ParseOneError::IMessedUp(format!("I messed up making the regex: {err}"))
        })?,
        Regex::new(r"(?s)\{\\bf Soluci\\'on:\}(.*)\\end\{document\}").map_err(|err| {
            ParseOneError::IMessedUp(format!("I messed up making the regex: {err}"))
        })?,
        Regex::new(r"(?s)\{\\bf Solución:\}(.*)\\end\{document\}").map_err(|err| {
            ParseOneError::IMessedUp(format!("I messed up making the regex: {err}"))
        })?,
    ];

    let solution = sol_regex
        .iter()
        .flat_map(|regex| regex.captures_iter(input))
        .next()
        .ok_or(ParseOneError::SolutionNotFound(id))?
        .get(1)
        .ok_or_else(|| {
            ParseOneError::IMessedUp(format!(
                "The captured group should have an entry, parsing problem {id}"
            ))
        })?
        .as_str();

    Ok(solution)
}

pub fn packages(data: &mut Data, input: &str) -> Result<(), ParseOneError> {
    Regex::new(r"\\usepackage\[(.*)\]\{(.*)}")
        .map_err(|err| ParseOneError::IMessedUp(format!("I messed up making the regex: {err}")))?
        .captures_iter(input)
        .for_each(|result| {
            let option = result.get(1).unwrap().as_str();
            let package = result.get(2).unwrap().as_str();
            if [
                "inputenc", "babel", "pim", "graphicx", "amssymb", "latexsym", "amsmath", "amsthm",
                "verbatim", "gensymb", "mathrsfs", "pgfplots", "textcomp", "tikz",
            ]
            .contains(&package)
            {
                return;
            }
            let use_statement = format!("\\usepackage[{option}]{{{package}}}");
            data.paquetes.push(use_statement);
        });

    let paquetes_2: String = Regex::new(r"\\usepackage\{(.*)}")
        .map_err(|err| ParseOneError::IMessedUp(format!("I messed up making the regex: {err}")))?
        .captures_iter(input)
        .flat_map(|result| {
            let packages = result.get(1).unwrap().as_str().split(',');
            packages
                .filter(|package| {
                    ![
                        "inputenc", "babel", "pim", "graphicx", "amssymb", "latexsym", "amsmath",
                        "amsthm", "verbatim", "gensymb", "mathrsfs", "pgfplots", "textcomp",
                        "tikz",
                    ]
                    .contains(package)
                })
                .map(|package| format!("\\usepackage{{{package}}}"))
        })
        .collect();

    let more_packages = paquetes_2.split('\n').map(std::borrow::ToOwned::to_owned);
    data.paquetes.extend(more_packages);

    Regex::new(r"\\usetikzlibrary\{(.*)}")
        .map_err(|err| ParseOneError::IMessedUp(format!("I messed up making the regex: {err}")))?
        .captures_iter(input)
        .for_each(|result| {
            let package = result.get(1).unwrap().as_str();
            data.paquetes.push(format!("\\usetikzlibrary{{{package}}}"));
        });

    Regex::new(r"\\pgfplotsset\{(.*)}")
        .map_err(|err| ParseOneError::IMessedUp(format!("I messed up making the regex: {err}")))?
        .captures_iter(input)
        .for_each(|result| {
            let package = result.get(1).unwrap().as_str();
            data.paquetes.push(format!("\\pgfplotsset{{{package}}}"));
        });

    data.sort_packages();

    Ok(())
}

pub fn find_info_from_template(
    id: usize,
    input: &str,
) -> Result<(Data, Vec<Fields>), ParseOneError> {
    let mut missing_data = vec![];
    let mut new_data = Data::new(id);
    for field in Fields::ALL {
        if !field.is_in_template() {
            continue;
        }
        let info = field.find(input).map_err(ParseOneError::IMessedUp)?;

        info.map_or_else(
            || {
                missing_data.push(field);
            },
            |content| new_data.set(content),
        );
    }
    Ok((new_data, missing_data))
}
