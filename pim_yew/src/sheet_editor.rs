use material_yew::checkbox::MatCheckbox;
use std::{collections::HashSet, fmt::Display, rc::Rc};
use web_sys::File;

use pim_lib::Data;
use yew::prelude::*;
use SheetTypes::{Jupiter, Mercury, Thematic, Uranus, Venus};

#[repr(usize)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum SheetTypes {
    Mercury,
    Venus,
    Jupiter,
    Uranus,
    Thematic,
}

impl From<SheetTypes> for &'static str {
    fn from(value: SheetTypes) -> Self {
        match value {
            Mercury => "Mercurio",
            Venus => "Venus",
            Jupiter => "Júpiter",
            Uranus => "Urano",
            Thematic => "Hoja temática",
        }
    }
}

impl Display for SheetTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &str = (*self).into();
        write!(f, "{name}")
    }
}

const N_TYPES: usize = 5;
const SHEETS: [SheetTypes; N_TYPES] = [Mercury, Venus, Jupiter, Uranus, Thematic];

pub struct Comp {
    present_sheets: [bool; N_TYPES],
    // Doesn't contain key means it's included: every
    // problem is included by default
    excluded_problems: HashSet<(SheetTypes, usize)>,
    previews: HashSet<usize>,
    template: Option<File>,
}

pub enum Msg {
    ShowPreview((usize, bool)),
    IncludeProblemToggle(SheetTypes, usize, bool),
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub db: Rc<Vec<Data>>,
    pub cart: Rc<Vec<usize>>,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            present_sheets: [true; N_TYPES],
            // TODO save this in local storage
            excluded_problems: HashSet::new(),
            previews: HashSet::new(),
            template: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowPreview((id, show)) => {
                log::info!("{id}, {show}");
                if show {
                    self.previews.insert(id);
                } else {
                    self.previews.remove(&id);
                }
            }
            Msg::IncludeProblemToggle(sheet, id, include) => {
                if include {
                    self.excluded_problems.remove(&(sheet, id));
                } else {
                    self.excluded_problems.insert((sheet, id));
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let headers: Html = SHEETS.into_iter().map(|x| html! {<th>{x}</th>}).collect();
        let rows: Html = ctx.props().cart.iter().map(|p| self.row(ctx, *p)).collect();
        html! {
            <table id="sheet-editor-table">
            <thead>
            <th>
            {"Problema"}
            </th>
            {headers}
            </thead>
            <tbody>
            {rows}
            </tbody>
            </table>
        }
    }
}

impl Comp {
    fn row(&self, ctx: &Context<Self>, id: usize) -> Html {
        let (problem_contents, title) = ctx
            .props()
            .db
            .iter()
            .find_map(|problem| {
                if problem.id == id {
                    Some((problem.enunciado.clone(), problem.titulo.clone()))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                log::error!("El problema {id} no está en la base de datos!");
                (String::new(), String::new())
            });

        let first_col = if self.previews.contains(&id) {
            let hide = button_callback!(ctx, Msg::ShowPreview, (id, false));

            html! {
            <td>
             <button onclick={hide} title="Mostrar el enunciado">
             <i class="fa-solid fa-eye-slash"></i>
             </button>
             {title}
                <crate::raw_html::Comp inner_html={problem_contents} tag="div" />
            </td>
            }
        } else {
            let show = button_callback!(ctx, Msg::ShowPreview, (id, true));
            html! {
            <td>
             <button onclick={show} title="Mostrar el enunciado">
             <i class="fa-solid fa-eye"></i>
             </button>
             {title}
            </td>
            }
        };
        let showing = self.present_sheets;
        let id_2 = id;
        let boxes: Html = SHEETS
            .into_iter()
            .filter(|s| showing[*s as usize])
            .map(|s| {
                let onchange = ctx
                    .link()
                    .callback(move |val| Msg::IncludeProblemToggle(s, id_2, val));
                let checked = !self.excluded_problems.contains(&(s, id));
                html! {
                <td>
                <MatCheckbox {onchange} {checked}/>
                </td>
                }
            })
            .collect();
        html! {
            <tr>
                {first_col}
                {boxes}
            </tr>
        }
    }
}
