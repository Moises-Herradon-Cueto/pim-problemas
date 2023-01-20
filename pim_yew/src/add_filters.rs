use std::fmt::Display;

use crate::field_selector::Comp as FieldSelect;

use material_yew::text_inputs::{MatTextField, TextFieldType};

use pim_lib::{Data, FieldContents, Fields};
use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Debug)]
pub struct Comp {
    field: Fields,
    string: String,
}

pub enum Msg {
    SearchField(String),
    FieldType(Fields),
    AddFilter,
    RemoveAll,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub filter_cb: Callback<FilterAction>,
}

#[derive(Debug)]
pub struct Filter {
    contents: FieldContents,
    str_contents: String,
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl Filter {
    pub fn new(field: Fields, contents: &str) -> Option<Self> {
        let field_contents = match field {
            Fields::Id => FieldContents::Id(contents.parse().ok()?),
            Fields::Title => FieldContents::Title(contents.to_lowercase()),
            Fields::Problem => FieldContents::Problem(contents.to_lowercase()),
            Fields::Topics => FieldContents::Topics(vec![contents.to_lowercase()]),
            Fields::Figures => FieldContents::Figures(vec![contents.to_lowercase()]),
            Fields::Difficulty => FieldContents::Difficulty(contents.parse().ok()?),
            Fields::Source => FieldContents::Source(contents.to_lowercase()),
            Fields::History => FieldContents::History(contents.to_lowercase()),
            Fields::Comments => FieldContents::Comments(contents.to_lowercase()),
            Fields::Year => FieldContents::Year(None),
            Fields::Packages => FieldContents::Packages(contents.to_lowercase()),
            Fields::Author => FieldContents::Author(contents.to_lowercase()),
            Fields::TexUrl => FieldContents::TexUrl(contents.to_lowercase()),
            Fields::PdfUrl => FieldContents::PdfUrl(contents.to_lowercase()),
        };
        Some(Self {
            contents: field_contents,
            str_contents: if matches!(field, Fields::Year) {
                contents.to_owned()
            } else {
                String::new()
            },
        })
    }

    pub fn passes(&self, data: &Data) -> bool {
        match &self.contents {
            FieldContents::Id(contents) => data.id == *contents,
            FieldContents::Title(contents) => data.titulo.to_lowercase().contains(contents),
            FieldContents::Problem(contents) => data.enunciado.to_lowercase().contains(contents),
            FieldContents::Topics(contents) => matches(
                contents.iter().map(String::as_str),
                &data.temas.iter().map(String::as_str),
            ),
            FieldContents::Figures(contents) => matches(
                contents.iter().map(String::as_str),
                &data.figuras.iter().map(String::as_str),
            ),
            FieldContents::Difficulty(contents) => data.dificultad == *contents,
            FieldContents::Source(contents) => data.fuente.to_lowercase().contains(contents),
            FieldContents::History(contents) => {
                if contents.is_empty() {
                    data.historial.is_empty()
                } else {
                    matches(contents.split(','), &data.historial.lines())
                }
            }
            FieldContents::Comments(contents) => {
                matches(contents.split(','), &data.comentarios.lines())
            }
            FieldContents::Year(_) => data
                .curso
                .map_or(true, |c| c.to_string().contains(&self.str_contents)),
            FieldContents::Packages(contents) => {
                matches(contents.split(','), &data.paquetes.lines())
            }
            FieldContents::TexUrl(contents) => data.tex_url.to_lowercase().contains(contents),
            FieldContents::PdfUrl(contents) => data.pdf_url.to_lowercase().contains(contents),
            FieldContents::Author(contents) => data.id_autor.to_lowercase().contains(contents),
        }
    }
}

fn matches<
    'a,
    'b,
    'c,
    T: IntoIterator<Item = &'a str>,
    S: 'c + Clone + IntoIterator<Item = &'c str>,
>(
    patterns: T,
    data: &'b S,
) -> bool {
    patterns.into_iter().all(|pattern| {
        data.clone()
            .into_iter()
            .any(|t| t.to_lowercase().contains(pattern))
    })
}

pub enum FilterAction {
    RemoveAll,
    Add(Filter),
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            field: Fields::Topics,
            string: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SearchField(string) => {
                self.string = string;
            }
            Msg::FieldType(field) => {
                self.field = field;
            }
            Msg::AddFilter => {
                let Some(filter) = Filter::new(self.field, &self.string)else {
                    log::info!("No es un filtro válido {self:?}");
                    return false;
                };
                ctx.props().filter_cb.emit(FilterAction::Add(filter));
            }
            Msg::RemoveAll => ctx.props().filter_cb.emit(FilterAction::RemoveAll),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::AddFilter
        });
        let onclick_2 = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::RemoveAll
        });
        let oninput = ctx.link().callback(Msg::SearchField);

        let select_cb = ctx.link().callback(Msg::FieldType);

        html! {
            <form id="filter_add">
            <FieldSelect  {select_cb}/>
            <MatTextField
                field_type={TextFieldType::Search}
                placeholder={Some(AttrValue::Owned(String::from("geometría")))}
                {oninput}
                />
            <button {onclick}>{"Añadir filtro"}</button>
            <button onclick={onclick_2}>{"Quitar los filtros"}</button>
            </form>
        }
    }
}
