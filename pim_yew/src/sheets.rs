use crate::raw_html;
use pim_lib::Data;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;
use yew::{prelude::*, virtual_dom::AttrValue};

use crate::extern_functions;

use FileType::{Pdf, Tex};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Planet {
    Mercurio,
    Venus,
    Jupiter,
    Urano,
}

impl Planet {
    const fn to_static_str(self) -> &'static str {
        match self {
            Self::Mercurio => "Mercurio",
            Self::Venus => "Venus",
            Self::Jupiter => "Júpiter",
            Self::Urano => "Urano",
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sheet {
    name: String,
    id: usize,
    year: u16,
    planet: Option<Planet>,
    tex_with_sols: Option<String>,
    tex_no_sols: Option<String>,
    pdf_with_sols: Option<String>,
    pdf_no_sols: Option<String>,
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
        file_type: FileType,
    },
    UpdateError(Solutions, usize, Option<String>),
    ReloadSheets,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub db: Rc<Vec<Data>>,
    pub sheets: Vec<Sheet>,
    pub reload_sheets_cb: Callback<()>,
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
            Msg::ReloadSheets => {
                ctx.props().reload_sheets_cb.emit(());
                false
            }
            Msg::UploadSheet {
                elt_id,
                sheet_id,
                index,
                with_solutions,
                file_type,
            } => {
                let with_solutions_2 = with_solutions;
                ctx.link()
                    .send_message(Msg::UpdateError(with_solutions, index, None));
                ctx.link().send_future_batch(async move {
                    let reply = extern_functions::upload_sheet(
                        elt_id.clone(),
                        sheet_id,
                        with_solutions_2 as u8,
                        file_type.to_string(),
                    )
                    .await;
                    let reply = reply.dyn_into::<Response>().unwrap();
                    let text_promise = reply.text().unwrap();
                    let text = JsFuture::from(text_promise)
                        .await
                        .unwrap()
                        .as_string()
                        .unwrap();
                    vec![
                        Msg::UpdateError(with_solutions, index, Some(text)),
                        Msg::ReloadSheets,
                    ]
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
#[repr(u8)]
pub enum Solutions {
    No = 0,
    Yes = 1,
}

#[derive(Clone, Copy)]
pub enum FileType {
    Tex,
    Pdf,
}

impl Display for Solutions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "con-soluciones"),
            Self::No => write!(f, "sin-soluciones"),
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
                    {self.doc_cell(ctx, sheet, Solutions::No, index)}
                </td>
                <td>
                    {self.doc_cell(ctx, sheet, Solutions::Yes, index)}
                </td>
            </tr>
        }
    }
    // Contiene el tex y el pdf, con/sin soluciones según
    // el valor de with_solutions
    fn doc_cell(
        &self,
        ctx: &Context<Self>,
        sheet: &Sheet,
        with_solutions: Solutions,
        index: usize,
    ) -> Html {
        let tex = match with_solutions {
            Solutions::Yes => &sheet.tex_with_sols,
            Solutions::No => &sheet.tex_no_sols,
        };
        let pdf = match with_solutions {
            Solutions::Yes => &sheet.pdf_with_sols,
            Solutions::No => &sheet.pdf_no_sols,
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
            {Self::file_upload(ctx, sheet, with_solutions, index, tex.as_ref(), Tex)}
            {Self::file_upload(ctx, sheet, with_solutions, index, pdf.as_ref(), Pdf)}
            {error_msg}
            </div>
        }
    }
    fn file_upload(
        ctx: &Context<Self>,
        sheet: &Sheet,
        with_solutions: Solutions,
        index: usize,
        url: Option<&String>,
        file_type: FileType,
    ) -> Html {
        let file = url.map_or_else(
            || {
                html! {<p>{AttrValue::from(format!("No hay {file_type} subido"))}</p>}
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
        let elt_id = format!("hoja-{}-{with_solutions}-{file_type}", sheet.id);

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
                    file_type,
                    index,
                }
            })
        };

        html! {
                    <div>
                    {file}
        <input type="file" id={AttrValue::from(elt_id)} class="file-upload" accept={file_type.accept_types()} />
                    <button {onclick}>{"Subir"}</button>
                    </div>
                }
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tex => write!(f, "tex"),
            Pdf => write!(f, "pdf"),
        }
    }
}

impl FileType {
    const fn accept_types(self) -> &'static str {
        match self {
            Tex => ".tex,.zip,application/zip,application/x-tex,text/x-tex",
            Pdf => ".pdf,application/pdf",
        }
    }
}
