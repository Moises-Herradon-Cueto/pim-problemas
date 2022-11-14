use std::borrow::Cow;
use std::fmt::Display;

use crate::Data;
use regex::Regex;
use serde::{Deserialize, Serialize};

use Fields::{
    Comments, Difficulty, History, Id, Packages, Problem, Solution, Source, Topics, Year,
};
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Fields {
    Id,
    Problem,
    Topics,
    Difficulty,
    Source,
    History,
    Comments,
    Year,
    Packages,
    Solution,
}

pub struct OutOfRange;

impl TryFrom<usize> for Fields {
    type Error = OutOfRange;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::ALL.get(value).copied().ok_or(OutOfRange)
    }
}

impl Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id => f.write_str("Id"),
            Problem => f.write_str("Enunciado"),
            Solution => f.write_str("Solución"),
            Topics => f.write_str("Temas"),
            Difficulty => f.write_str("Dificultad"),
            Source => f.write_str("Fuente"),
            History => f.write_str("Historial"),
            Comments => f.write_str("Comentarios"),
            Year => f.write_str("Curso"),
            Packages => f.write_str("Paquetes usados"),
        }
    }
}

impl Fields {
    pub const N: usize = 10;
    pub const ALL: [Self; Self::N] = [
        Id, Problem, Topics, Difficulty, Source, History, Comments, Year, Packages, Solution,
    ];

    #[must_use]
    pub fn get_string(self, data: &Data) -> Cow<str> {
        match self {
            Id => Cow::Owned(data.id.to_string()),
            Problem => Cow::Borrowed(&data.enunciado),
            Solution => Cow::Borrowed("No están guardadas las soluciones"),
            Topics => Cow::Owned(data.temas.join(", ")),
            Difficulty => Cow::Owned(data.dificultad.to_string()),
            Source => Cow::Borrowed(&data.fuente),
            History => Cow::Owned(data.historial.join("\n")),
            Comments => Cow::Owned(data.comentarios.join("\n")),
            Year => Cow::Owned(data.curso.clone().unwrap_or_default()),
            Packages => Cow::Owned(data.paquetes.join("\n")),
        }
    }

    #[must_use]
    pub fn get(self, data: &Data) -> FieldContentsRef {
        match self {
            Id => FieldContentsRef::Id(data.id),
            Problem => FieldContentsRef::Problem(&data.enunciado),
            Solution => FieldContentsRef::Solution,
            Difficulty => FieldContentsRef::Difficulty(data.dificultad),
            Topics => FieldContentsRef::Topics(&data.temas),
            Source => FieldContentsRef::Source(&data.fuente),
            History => FieldContentsRef::History(&data.historial),
            Comments => FieldContentsRef::Comments(&data.comentarios),
            Year => FieldContentsRef::Year(&data.curso),
            Packages => FieldContentsRef::Packages(&data.paquetes),
        }
    }

    #[must_use]
    pub const fn is_in_template(self) -> bool {
        !matches!(self, Self::Solution)
    }

    #[must_use]
    pub(crate) fn regex(self) -> Regex {
        let attempt = match self {
            Problem => Regex::new(r"(?s)\\begin\{ejer\}\s*(.*?)\s*\\end\{ejer\}"),
            Solution => Regex::new(r"$."),
            Topics => Regex::new(r"\\temas\{\s*(.*?)\s*\}"),
            Difficulty => Regex::new(r"\\dificultad\{\s*(.*?)\s*\}"),
            Source => Regex::new(r"\\fuente\{\s*(.*?)\s*\}"),
            History => Regex::new(r"\\historial\{\s*(.*?)\s*\}"),
            Comments => Regex::new(r"\\comentarios\{\s*(.*?)\s*\}"),
            Year => Regex::new(r"\\curso\{\s*(.*?)\s*\}"),
            Packages => Regex::new(r"(?s)%%% Paquetes extra\s*(.*?)\s*%%% Fin de paquetes extra"),
            Id => Regex::new(r"\\id\{\s*(.*?)\s*\}"),
        };
        attempt.expect("I messed up making the regex")
    }

    const fn empty(self) -> FieldContents {
        match self {
            Id => FieldContents::Id(usize::MAX),
            Problem => FieldContents::Problem(String::new()),
            Solution => FieldContents::Solution,
            Topics => FieldContents::Topics(Vec::new()),
            Difficulty => FieldContents::Difficulty(u8::MAX),
            Source => FieldContents::Source(String::new()),
            History => FieldContents::History(Vec::new()),
            Comments => FieldContents::Comments(Vec::new()),
            Year => FieldContents::Year(None),
            Packages => FieldContents::Packages(Vec::new()),
        }
    }

    fn parse(self, input: &str) -> Result<FieldContents, String> {
        let input = input.trim();
        if input.is_empty() || input == "%" {
            return Ok(self.empty());
        }
        match self {
            Id => Ok(FieldContents::Id(
                input
                    .parse()
                    .map_err(|err| format!("Error parsing: {err}"))?,
            )),
            Problem => Ok(FieldContents::Problem(input.to_owned())),
            Solution => Ok(FieldContents::Solution),
            Topics => Ok(FieldContents::Topics(
                input
                    .split(',')
                    .map(|topic| topic.trim().to_owned())
                    .collect(),
            )),
            Difficulty => Ok(FieldContents::Difficulty(
                input
                    .parse()
                    .map_err(|err| format!("Error parsing: {err}"))?,
            )),
            Source => Ok(FieldContents::Source(input.to_owned())),
            History => Ok(FieldContents::History(
                input
                    .split(',')
                    .map(|topic| topic.trim().to_owned())
                    .collect(),
            )),
            Comments => Ok(FieldContents::Comments(
                input
                    .split(',')
                    .map(|topic| topic.trim().to_owned())
                    .collect(),
            )),
            Year => {
                if input.is_empty() || input == "%" {
                    Ok(FieldContents::Year(None))
                } else {
                    Ok(FieldContents::Year(Some(input.to_owned())))
                }
            }
            Packages => Ok(FieldContents::Packages(
                input
                    .split('\n')
                    .map(|topic| topic.trim().to_owned())
                    .collect(),
            )),
        }
    }

    pub(crate) fn find(self, input: &str) -> Result<Option<FieldContents>, String> {
        if !self.is_in_template() {
            return Ok(None);
        }
        let regex = self.regex();
        let capture = regex.captures_iter(input).next();
        let Some(capture) = capture else {return Ok(None)};
        let found_info = capture
            .get(1)
            .expect("Messed up the regex, there should be one capture group")
            .as_str();
        Ok(Some(self.parse(found_info)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldContents {
    Id(usize),
    Problem(String),
    Solution,
    Topics(Vec<String>),
    Difficulty(u8),
    Source(String),
    History(Vec<String>),
    Comments(Vec<String>),
    Year(Option<String>),
    Packages(Vec<String>),
}

#[derive(PartialEq, Eq)]
pub enum FieldContentsRef<'a> {
    Id(usize),
    Problem(&'a str),
    Solution,
    Topics(&'a [String]),
    Difficulty(u8),
    Source(&'a str),
    History(&'a [String]),
    Comments(&'a [String]),
    Year(&'a Option<String>),
    Packages(&'a [String]),
}

impl<'a> PartialOrd for FieldContentsRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for FieldContentsRef<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (FieldContentsRef::Id(id_1), FieldContentsRef::Id(id_2)) => id_1.cmp(id_2),
            (FieldContentsRef::Problem(x_1), FieldContentsRef::Problem(x_2))
            | (FieldContentsRef::Source(x_1), FieldContentsRef::Source(x_2)) => x_1.cmp(x_2),
            (FieldContentsRef::Topics(x_1), FieldContentsRef::Topics(x_2))
            | (FieldContentsRef::History(x_1), FieldContentsRef::History(x_2))
            | (FieldContentsRef::Comments(x_1), FieldContentsRef::Comments(x_2))
            | (FieldContentsRef::Packages(x_1), FieldContentsRef::Packages(x_2)) => x_1.cmp(x_2),
            (FieldContentsRef::Difficulty(x_1), FieldContentsRef::Difficulty(x_2)) => x_1.cmp(x_2),
            (FieldContentsRef::Year(x_1), FieldContentsRef::Year(x_2)) => x_1.cmp(x_2),
            (_, _) => std::cmp::Ordering::Equal,
        }
    }
}

impl<'a> FieldContentsRef<'a> {
    #[must_use]
    pub fn to_owned(&self) -> FieldContents {
        match self {
            FieldContentsRef::Id(x) => FieldContents::Id(*x),
            FieldContentsRef::Difficulty(x) => FieldContents::Difficulty(*x),
            FieldContentsRef::Problem(x) => FieldContents::Problem((*x).to_owned()),
            FieldContentsRef::Solution => FieldContents::Solution,
            FieldContentsRef::Source(x) => FieldContents::Source((*x).to_owned()),
            FieldContentsRef::Topics(x) => FieldContents::Topics((*x).to_vec()),
            FieldContentsRef::History(x) => FieldContents::History((*x).to_vec()),
            FieldContentsRef::Comments(x) => FieldContents::Comments((*x).to_vec()),
            FieldContentsRef::Packages(x) => FieldContents::Packages((*x).to_vec()),
            FieldContentsRef::Year(x) => FieldContents::Year(x.as_ref().cloned()),
        }
    }
}
impl FieldContents {
    pub(crate) fn set(self, data: &mut Data) {
        match self {
            Self::Id(content) => data.id = content,
            Self::Problem(content) => data.enunciado = content,
            Self::Solution => println!("La solución no está guardada"),
            Self::Difficulty(content) => data.dificultad = content,
            Self::Topics(content) => data.temas = content,
            Self::Source(content) => data.fuente = content,
            Self::History(content) => data.historial = content,
            Self::Comments(content) => data.comentarios = content,
            Self::Year(content) => data.curso = content,
            Self::Packages(content) => data.paquetes = content,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        use FieldContents::{
            Comments, Difficulty, History, Id, Packages, Problem, Solution, Source, Topics, Year,
        };
        match self {
            Id(x) => *x == usize::MAX,
            Difficulty(x) => *x == u8::MAX,
            Problem(x) | Source(x) => x.is_empty() || x == "%",
            Topics(x) | History(x) | Comments(x) | Packages(x) => x.is_empty(),
            Year(x) => x.is_none(),
            Solution => true,
        }
    }
}

impl From<&FieldContents> for Fields {
    fn from(value: &FieldContents) -> Self {
        match value {
            FieldContents::Id(_) => Self::Id,
            FieldContents::Problem(_) => Self::Problem,
            FieldContents::Solution => Self::Solution,
            FieldContents::Topics(_) => Self::Topics,
            FieldContents::Difficulty(_) => Self::Difficulty,
            FieldContents::Source(_) => Self::Source,
            FieldContents::History(_) => Self::History,
            FieldContents::Comments(_) => Self::Comments,
            FieldContents::Year(_) => Self::Year,
            FieldContents::Packages(_) => Self::Packages,
        }
    }
}

impl Display for FieldContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FieldContents::{
            Comments, Difficulty, History, Id, Packages, Problem, Solution, Source, Topics, Year,
        };
        let string = match self {
            Id(x) => Cow::Owned(x.to_string()),
            Difficulty(x) => Cow::Owned(x.to_string()),
            Problem(x) | Source(x) => Cow::Borrowed(x),
            Topics(x) | History(x) | Comments(x) | Packages(x) => Cow::Owned(x.join(",")),
            Year(x) => x
                .as_ref()
                .map_or_else(|| Cow::Owned(String::new()), Cow::Borrowed),
            Solution => Cow::Owned(String::new()),
        };
        write!(f, "{}: {string}", Fields::from(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::Fields;

    #[test]
    fn fields_list() {
        for field in Fields::ALL {
            assert_eq!(field, Fields::ALL[field as usize]);
        }
    }
}
