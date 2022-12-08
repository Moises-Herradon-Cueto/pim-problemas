use std::fmt::Display;
use std::mem;
use std::{borrow::Cow, slice};

use crate::Data;
use clap::ValueEnum;
use regex::Regex;
use serde::{Deserialize, Serialize};

use Fields::{
    Author, Comments, Difficulty, Figures, History, Id, Packages, Problem, Source, Title, Topics,
    Url, Year,
};
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, ValueEnum, enum_utils::FromStr,
)]
#[repr(u8)]
pub enum Fields {
    Id,
    Title,
    Problem,
    Topics,
    Difficulty,
    Source,
    History,
    Comments,
    Year,
    Packages,
    Author,
    Url,
    Figures,
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
            Title => f.write_str("Título"),
            Id => f.write_str("Id"),
            Problem => f.write_str("Enunciado"),
            Topics => f.write_str("Temas"),
            Difficulty => f.write_str("Dificultad"),
            Source => f.write_str("Fuente"),
            History => f.write_str("Historial"),
            Comments => f.write_str("Comentarios"),
            Year => f.write_str("Curso"),
            Packages => f.write_str("Paquetes usados"),
            Author => f.write_str("Proponente"),
            Url => f.write_str("Archivo"),
            Figures => f.write_str("Figuras"),
        }
    }
}

impl Fields {
    pub const N: usize = 13;
    pub const ALL: [Self; Self::N] = [
        Id, Title, Problem, Topics, Difficulty, Source, History, Comments, Year, Packages, Author,
        Url, Figures,
    ];

    #[must_use]
    pub const fn db_label(self) -> &'static str {
        match self {
            Title => "titulo",
            Id => "id",
            Problem => "enunciado",
            Topics => "temas",
            Difficulty => "dificultad",
            Source => "procedencia",
            History => "hojas",
            Comments => "comentarios",
            Year => "curso",
            Packages => "preambulo usados",
            Author => "id_autor",
            Url => "url_tex",
            Figures => "figuras",
        }
    }

    #[must_use]
    pub const fn has_own_table(self) -> bool {
        matches!(self, Topics | History | Figures)
    }

    #[must_use]
    pub fn get_string(self, data: &Data) -> Cow<str> {
        match self {
            Id => Cow::Owned(data.id.to_string()),
            Problem => Cow::Borrowed(&data.enunciado),
            Title => Cow::Borrowed(&data.titulo),
            Topics => Cow::Owned(data.temas.join(", ")),
            Figures => Cow::Owned(data.figuras.join(", ")),
            Difficulty => Cow::Owned(data.dificultad.to_string()),
            Source => Cow::Borrowed(&data.fuente),
            History => Cow::Borrowed(&data.historial),
            Comments => Cow::Borrowed(&data.comentarios),
            Packages => Cow::Borrowed(&data.paquetes),
            Year => Cow::Borrowed(&data.curso),
            Author => Cow::Borrowed(&data.id_autor),
            Url => Cow::Borrowed(&data.url),
        }
    }

    #[must_use]
    pub fn get(self, data: &Data) -> FieldContentsRef {
        match self {
            Id => FieldContentsRef::Id(data.id),
            Title => FieldContentsRef::Title(&data.titulo),
            Problem => FieldContentsRef::Problem(&data.enunciado),
            Difficulty => FieldContentsRef::Difficulty(data.dificultad),
            Topics => FieldContentsRef::Topics(&data.temas),
            Source => FieldContentsRef::Source(&data.fuente),
            History => FieldContentsRef::History(&data.historial),
            Comments => FieldContentsRef::Comments(&data.comentarios),
            Year => FieldContentsRef::Year(&data.curso),
            Packages => FieldContentsRef::Packages(&data.paquetes),
            Author => FieldContentsRef::Author(&data.id_autor),
            Url => FieldContentsRef::Url(&data.url),
            Figures => FieldContentsRef::Figures(&data.figuras),
        }
    }

    #[must_use]
    pub const fn is_in_template(self) -> bool {
        true
    }

    #[must_use]
    pub(crate) fn regex(self) -> Regex {
        let attempt = match self {
            Problem => Regex::new(r"(?s)\\begin\{ejer\}\s*(.*?)\s*\\end\{ejer\}"),
            Topics => Regex::new(r"\\temas\{\s*(.*?)\s*\}"),
            Difficulty => Regex::new(r"\\dificultad\{\s*(.*?)\s*\}"),
            Source => Regex::new(r"\\fuente\{\s*(.*?)\s*\}"),
            History => Regex::new(r"\\historial\{\s*(.*?)\s*\}"),
            Comments => Regex::new(r"\\comentarios\{\s*(.*?)\s*\}"),
            Year => Regex::new(r"\\curso\{\s*(.*?)\s*\}"),
            Author => Regex::new(r"\\proponente\{\s*(.*?)\s*\}"),
            Url => Regex::new(r"\\archivo\{\s*(.*?)\s*\}"),
            Title => Regex::new(r"\\titulo\{\s*(.*?)\s*\}"),
            Packages => Regex::new(r"(?s)%%% Paquetes extra\s*(.*?)\s*%%% Fin de paquetes extra"),
            Id => Regex::new(r"\\id\{\s*(.*?)\s*\}"),
            Figures => Regex::new(r"\\includegraphics\s*(?:\[[^\]]*\])?\s*\{\s*(.*?)\s*\}"),
        };
        attempt.expect("I messed up making the regex")
    }

    const fn empty(self) -> FieldContents {
        match self {
            Id => FieldContents::Id(usize::MAX),
            Problem => FieldContents::Problem(String::new()),
            Topics => FieldContents::Topics(Vec::new()),
            Figures => FieldContents::Figures(Vec::new()),
            Difficulty => FieldContents::Difficulty(u8::MAX),
            Source => FieldContents::Source(String::new()),
            History => FieldContents::History(String::new()),
            Comments => FieldContents::Comments(String::new()),
            Year => FieldContents::Year(String::new()),
            Packages => FieldContents::Packages(String::new()),
            Author => FieldContents::Author(String::new()),
            Url => FieldContents::Url(String::new()),
            Title => FieldContents::Title(String::new()),
        }
    }

    #[must_use]
    pub const fn is_optional(self) -> bool {
        matches!(self, History | Comments | Packages)
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// it doesn't parse correctly
    pub fn parse(self, input: &str) -> Result<FieldContents, String> {
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
            Title => Ok(FieldContents::Title(input.to_owned())),
            Topics => Ok(FieldContents::Topics(
                input
                    .split(&[',', '\n'])
                    .map(|topic| topic.trim().to_owned())
                    .collect(),
            )),
            Figures => Ok(FieldContents::Topics(
                input
                    .split(&[',', '\n'])
                    .map(|figure| figure.trim().to_owned())
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
                    Ok(FieldContents::Year(String::new()))
                } else {
                    Ok(FieldContents::Year(input.to_owned()))
                }
            }
            Packages => Ok(FieldContents::Packages(
                input
                    .split('\n')
                    .filter_map(|topic| {
                        let trim = topic.trim();
                        if trim.is_empty() {
                            None
                        } else {
                            Some(trim.to_owned())
                        }
                    })
                    .collect(),
            )),
            Author => Ok(FieldContents::Author(input.trim().to_owned())),
            Url => Ok(FieldContents::Url(input.trim().to_owned())),
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum FieldContents {
    Id(usize),
    Title(String),
    Problem(String),
    Topics(Vec<String>),
    Difficulty(u8),
    Source(String),
    History(String),
    Comments(String),
    Year(String),
    Packages(String),
    Author(String),
    Url(String),
    Figures(Vec<String>),
}

#[derive(PartialEq, Eq)]
pub enum FieldContentsRef<'a> {
    Id(usize),
    Title(&'a str),
    Problem(&'a str),
    Topics(&'a [String]),
    Difficulty(u8),
    Source(&'a str),
    History(&'a str),
    Comments(&'a str),
    Year(&'a str),
    Packages(&'a str),
    Author(&'a str),
    Url(&'a str),
    Figures(&'a [String]),
}

pub enum ContentsIter<'a> {
    Single(&'a str),
    Owned(String),
    Multi(slice::Iter<'a, String>),
}

impl<'a> Iterator for ContentsIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ContentsIter::Single(x) => {
                if x.is_empty() {
                    None
                } else {
                    let out = mem::take(x);
                    Some(Cow::Borrowed(out))
                }
            }
            ContentsIter::Owned(x) => {
                if x.is_empty() {
                    None
                } else {
                    let out = mem::take(x);
                    Some(Cow::Owned(out))
                }
            }
            ContentsIter::Multi(i) => Some(Cow::Borrowed(i.next()?)),
        }
    }
}

impl<'a> IntoIterator for FieldContentsRef<'a> {
    type Item = Cow<'a, str>;

    type IntoIter = ContentsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            FieldContentsRef::Id(x) => Self::IntoIter::Owned(x.to_string()),
            FieldContentsRef::Difficulty(x) => Self::IntoIter::Owned(x.to_string()),
            FieldContentsRef::Title(x)
            | FieldContentsRef::Problem(x)
            | FieldContentsRef::Source(x)
            | FieldContentsRef::History(x)
            | FieldContentsRef::Comments(x)
            | FieldContentsRef::Year(x)
            | FieldContentsRef::Packages(x)
            | FieldContentsRef::Author(x)
            | FieldContentsRef::Url(x) => Self::IntoIter::Single(x),
            FieldContentsRef::Figures(x) | FieldContentsRef::Topics(x) => {
                Self::IntoIter::Multi(x.iter())
            }
        }
    }
}

impl<'a> FieldContentsRef<'a> {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        use FieldContentsRef::{
            Author, Comments, Difficulty, Figures, History, Id, Packages, Problem, Source, Title,
            Topics, Url, Year,
        };
        match self {
            Id(x) => *x == usize::MAX,
            Difficulty(x) => *x == u8::MAX,
            Problem(x) | Title(x) | Source(x) | History(x) | Comments(x) | Packages(x)
            | Year(x) | Author(x) | Url(x) => x.is_empty() || *x == "%",
            Topics(x) | Figures(x) => x.is_empty(),
        }
    }
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
            | (FieldContentsRef::Source(x_1), FieldContentsRef::Source(x_2))
            | (FieldContentsRef::History(x_1), FieldContentsRef::History(x_2))
            | (FieldContentsRef::Comments(x_1), FieldContentsRef::Comments(x_2))
            | (FieldContentsRef::Packages(x_1), FieldContentsRef::Packages(x_2))
            | (FieldContentsRef::Year(x_1), FieldContentsRef::Year(x_2)) => x_1.cmp(x_2),
            (FieldContentsRef::Topics(x_1), FieldContentsRef::Topics(x_2)) => x_1.cmp(x_2),
            (FieldContentsRef::Difficulty(x_1), FieldContentsRef::Difficulty(x_2)) => x_1.cmp(x_2),
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
            FieldContentsRef::Title(x) => FieldContents::Title((*x).to_owned()),
            FieldContentsRef::Source(x) => FieldContents::Source((*x).to_owned()),
            FieldContentsRef::Topics(x) => FieldContents::Topics((*x).to_vec()),
            FieldContentsRef::Figures(x) => FieldContents::Figures((*x).to_vec()),
            FieldContentsRef::History(x) => FieldContents::History((*x).to_owned()),
            FieldContentsRef::Comments(x) => FieldContents::Comments((*x).to_owned()),
            FieldContentsRef::Packages(x) => FieldContents::Packages((*x).to_owned()),
            FieldContentsRef::Year(x) => FieldContents::Year((*x).to_owned()),
            FieldContentsRef::Url(x) => FieldContents::Url((*x).to_owned()),
            FieldContentsRef::Author(x) => FieldContents::Author((*x).to_owned()),
        }
    }
}
impl FieldContents {
    pub(crate) fn set(self, data: &mut Data) {
        match self {
            Self::Id(content) => data.id = content,
            Self::Problem(content) => data.enunciado = content,
            Self::Title(content) => data.titulo = content,
            Self::Difficulty(content) => data.dificultad = content,
            Self::Topics(content) => data.temas = content,
            Self::Figures(content) => data.figuras = content,
            Self::Source(content) => data.fuente = content,
            Self::History(content) => data.historial = content,
            Self::Comments(content) => data.comentarios = content,
            Self::Year(content) => data.curso = content,
            Self::Packages(content) => data.paquetes = content,
            Self::Author(content) => data.id_autor = content,
            Self::Url(content) => data.url = content,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        use FieldContents::{
            Author, Comments, Difficulty, Figures, History, Id, Packages, Problem, Source, Title,
            Topics, Url, Year,
        };
        match self {
            Id(x) => *x == usize::MAX,
            Difficulty(x) => *x == u8::MAX,
            Title(x) | Problem(x) | Source(x) | History(x) | Comments(x) | Packages(x)
            | Year(x) | Author(x) | Url(x) => x.is_empty() || x == "%",
            Topics(x) | Figures(x) => x.is_empty(),
        }
    }

    #[must_use]
    pub fn string_contents(&self) -> Cow<String> {
        use FieldContents::{
            Author, Comments, Difficulty, Figures, History, Id, Packages, Problem, Source, Title,
            Topics, Url, Year,
        };
        match self {
            Id(x) => Cow::Owned(x.to_string()),
            Difficulty(x) => Cow::Owned(x.to_string()),
            Title(x) | Problem(x) | Source(x) | History(x) | Comments(x) | Packages(x)
            | Year(x) | Author(x) | Url(x) => Cow::Borrowed(x),
            Topics(x) | Figures(x) => Cow::Owned(x.join(",")),
        }
    }
}

impl From<&FieldContents> for Fields {
    fn from(value: &FieldContents) -> Self {
        match value {
            FieldContents::Id(_) => Self::Id,
            FieldContents::Title(_) => Self::Title,
            FieldContents::Problem(_) => Self::Problem,
            FieldContents::Topics(_) => Self::Topics,
            FieldContents::Difficulty(_) => Self::Difficulty,
            FieldContents::Source(_) => Self::Source,
            FieldContents::History(_) => Self::History,
            FieldContents::Comments(_) => Self::Comments,
            FieldContents::Year(_) => Self::Year,
            FieldContents::Packages(_) => Self::Packages,
            FieldContents::Author(_) => Self::Author,
            FieldContents::Url(_) => Self::Url,
            FieldContents::Figures(_) => Self::Figures,
        }
    }
}

impl Display for FieldContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self.string_contents();
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
