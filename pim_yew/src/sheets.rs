use crate::raw_html;
use pim_lib::Data;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;
use yew::{prelude::*, virtual_dom::AttrValue};

use crate::extern_functions;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Planet {
    Mercury,
    Venus,
    Jupiter,
    Uranus,
}

impl Planet {
    const fn to_static_str(self) -> &'static str {
        match self {
            Self::Mercury => "Mercurio",
            Self::Venus => "Venus",
            Self::Jupiter => "Júpiter",
            Self::Uranus => "Urano",
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sheet {
    name: String,
    id: usize,
    year: u16,
    planet: Option<Planet>,
    url_with_sols: Option<String>,
    url_no_sols: Option<String>,
}

pub struct Comp {
    errors_sols: Vec<Option<String>>,
    errors_no_sols: Vec<Option<String>>,
}

pub enum Msg {
    UploadSheet {
        elt_id: String,
        sheet_id: usize,
        index: usize,
        with_solutions: Solutions,
    },
    UpdateError(Solutions, usize, Option<String>),
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub db: Rc<Vec<Data>>,
    pub sheets: Vec<Sheet>,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let n = ctx.props().sheets.len();
        Self {
            errors_no_sols: vec![None; n],
            errors_sols: vec![None; n],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UploadSheet {
                elt_id,
                sheet_id,
                index,
                with_solutions,
            } => {
                let with_solutions_bool = bool::from(with_solutions);
                ctx.link()
                    .send_message(Msg::UpdateError(with_solutions, index, None));
                ctx.link().send_future(async move {
                    let reply = extern_functions::upload_sheet(
                        elt_id.clone(),
                        sheet_id,
                        with_solutions_bool,
                    )
                    .await;
                    let reply = reply.dyn_into::<Response>().unwrap();
                    let text_promise = reply.text().unwrap();
                    let text = JsFuture::from(text_promise)
                        .await
                        .unwrap()
                        .as_string()
                        .unwrap();
                    log::info!("text");
                    Msg::UpdateError(with_solutions, index, Some(text))
                });
                false
            }
            Msg::UpdateError(with_sols, i, msg) => {
                let list = match with_sols {
                    Solutions::Yes => &mut self.errors_sols,
                    Solutions::No => &mut self.errors_no_sols,
                };
                list[i] = msg;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let filas: Html = ctx
            .props()
            .sheets
            .iter()
            .enumerate()
            .map(|(index, sheet)| self.make_row(ctx, sheet, index))
            .collect();

        html! {
        <div id="sheets-table-container">
            <table id="db-table">
            <thead>
            <th>{"Id"}</th>
            <th>{"Título"}</th>
            <th>{"Año"}</th>
            <th>{"Planeta"}</th>
            <th>{"Hoja sin soluciones"}</th>
            <th>{"Hoja con soluciones"}</th>
            </thead>
                {filas}
            </table>
            </div>
        }
    }
}

#[derive(Clone, Copy)]
pub enum Solutions {
    Yes,
    No,
}

impl Display for Solutions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "sin-soluciones"),
            Self::No => write!(f, "con-soluciones"),
        }
    }
}

impl From<Solutions> for bool {
    fn from(value: Solutions) -> Self {
        matches!(value, Solutions::Yes)
    }
}

impl Comp {
    fn make_row(&self, ctx: &Context<Self>, sheet: &Sheet, index: usize) -> Html {
        let edit_button = html! {}; // html! {<button>{"Este botón aún no hace nada"}</button>};
        html! {
            <tr>
                <td>
                    {sheet.id}
                    {edit_button}
                </td>
                <td>
                    {sheet.name.clone()}
                </td>
                <td>
                    {sheet.year}
                </td>
                <td>
                    {sheet.planet.map_or("Todos", Planet::to_static_str)}
                </td>
                <td>
                    {self.file_upload(ctx, sheet, Solutions::No, index)}
                </td>
                <td>
                    {self.file_upload(ctx, sheet, Solutions::Yes, index)}
                </td>
            </tr>
        }
    }
    fn file_upload(
        &self,
        ctx: &Context<Self>,
        sheet: &Sheet,
        with_solutions: Solutions,
        index: usize,
    ) -> Html {
        let url = match with_solutions {
            Solutions::Yes => &sheet.url_with_sols,
            Solutions::No => &sheet.url_no_sols,
        };
        let file = url.as_ref().map_or_else(
            || {
                html! {<p>{"No hay nada subido"}</p>}
            },
            |url| {
                let extension = url.split('.').fold("", |_, s| s);
                let file_name =
                    AttrValue::from(format!("{}-{with_solutions}.{extension}", sheet.name));
                html! {
                <a href={AttrValue::from(url.clone())} download={file_name.clone()}>{file_name}</a>
                }
            },
        );
        let elt_id = format!("hoja-{}-{with_solutions}-archivo", sheet.id);

        let sheet_id = sheet.id;
        let with_solutions_2 = with_solutions;
        let onclick = {
            let elt_id_2 = elt_id.clone();
            ctx.link().callback(move |e: MouseEvent| {
                e.prevent_default();
                Msg::UploadSheet {
                    elt_id: elt_id_2.clone(),
                    sheet_id,
                    with_solutions: with_solutions_2,
                    index,
                }
            })
        };
        let error_list = match with_solutions {
            Solutions::Yes => &self.errors_sols,
            Solutions::No => &self.errors_no_sols,
        };
        let error_msg = error_list[index].as_ref().map_or_else(
            || html! {},
            |msg| html! {<raw_html::Comp tag="p" inner_html={msg.clone()}/>},
        );

        html! {
            <div>
            {file}
            <input type="file" id={AttrValue::from(elt_id)} class="file-upload" />
            {error_msg}
            <button {onclick}>{"Subir"}</button>
            </div>
        }
    }
}
