use std::rc::Rc;

use crate::add_filters::{Comp as FilterAdd, Filter, FilterAction};
use crate::column_select::Comp as ColumnSelect;
use crate::edit_entry::Comp as EditEntry;
use crate::field_display::Comp as FieldDisplay;
use crate::field_selector::Comp as FieldSelect;
use crate::result_range::{self, Comp as RangeSelector};
use crate::typeset;
use log::{warn, error};
use material_yew::MatIconButtonToggle;
use pim_lib::{Data, Fields, ParseOneError};
use web_sys::window;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use SpecialWindow::*;

pub struct ViewDb {
    view: Vec<Data>,
    window: Vec<Data>,
    shown_fields: [bool; Fields::N],
    char_length: usize,
    filters: Vec<Filter>,
    sort: Sort,
    error: Option<ParseOneError>,
    range: (usize, usize),
    cached_range: (usize, usize),
    special_window: SpecialWindow,
}

enum SpecialWindow {
    Normal,
    ViewingPdf(usize),
    Editing(usize),
}

struct Sort {
    by: Fields,
    ascending: bool,
}

impl Default for Sort {
    fn default() -> Self {
        Self {
            by: Fields::Id,
            ascending: true,
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum Msg {
    View(bool, Fields),
    EditFilter(FilterAction),
    SortField(Fields),
    SortAsc(bool),
    EditInfo(Data),
    SetError(ParseOneError),
    Range(usize, result_range::Which),
    Edit(usize),
    TryDelete { id: usize, title: String },
    BackToNormal,
    ReloadDb,
    ViewPdf(usize),
    AddToCart(usize),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub db: Rc<Vec<Data>>,
    pub reload_db_cb: Callback<()>,
    pub edit_cb: Callback<Data>,
    pub delete_cb: Callback<usize>,
    pub add_to_cart: Callback<usize>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.db, &other.db) && self.reload_db_cb == other.reload_db_cb
    }
}

impl Component for ViewDb {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut shown_fields = [true; Fields::N];
        for (i, f) in Fields::ALL.into_iter().enumerate() {
            shown_fields[i] = f.is_in_template();
        }
        // Hide id and title, file name and figures
        shown_fields[0] = false;
        shown_fields[1] = false;
        shown_fields[11] = false;
        shown_fields[12] = false;
        let mut output = Self {
            view: vec![],
            window: vec![],
            shown_fields,
            char_length: 100,
            filters: vec![],
            sort: Sort::default(),
            error: None,
            range: (0, 20),
            cached_range: (0, 0),
            special_window: SpecialWindow::Normal,
        };

        output.calculate_view(ctx);

        output
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.calculate_view(ctx);

        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddToCart(id) => {
                ctx.props().add_to_cart.emit(id);
                return false;
            }
            Msg::TryDelete { id, title } => {
                let response = window().unwrap().confirm_with_message(&format!(
                    "¿Seguro que quieres borrar el problema {title}?"
                ));
                match response {
                    Ok(true) => {
                        ctx.props().delete_cb.emit(id);
                    }
                    Ok(false) => {}
                    Err(err) => {
                        log::error!("{err:?}");
                    }
                }
            }
            Msg::BackToNormal => {
                self.stop_editing(ctx);
            }
            Msg::SetError(err) => {
                self.error = Some(err);
            }
            Msg::ReloadDb => {
                ctx.props().reload_db_cb.emit(());
                return false;
            }
            Msg::EditInfo(data) => {
                ctx.props().edit_cb.emit(data);
                self.stop_editing(ctx);
            }
            Msg::View(show, field) => {
                self.shown_fields[field as usize] = show;
            }
            Msg::EditFilter(FilterAction::RemoveAll) => {
                self.filters = vec![];
                self.calculate_view(ctx);
            }
            Msg::EditFilter(FilterAction::Add(filter)) => {
                self.filters.push(filter);
                self.calculate_view(ctx);
            }
            Msg::SortField(field) => {
                self.sort.by = field;
                self.calculate_view(ctx);
            }
            Msg::SortAsc(bool) => {
                self.sort.ascending = bool;
                self.calculate_view(ctx);
            }
            Msg::Range(x, result_range::Which::Start) => {
                self.range.0 = x;
                self.calculate_window();
            }
            Msg::Range(x, result_range::Which::End) => {
                self.range.1 = x;
                self.calculate_window();
            }
            Msg::Edit(id) => {
                self.special_window = Editing(id);
                self.cached_range = self.range;
            }
            Msg::ViewPdf(id) => {
                self.special_window = ViewingPdf(id);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.special_window {
            Normal => self.view_all(ctx),
            ViewingPdf(id) => self.view_pdf(ctx, id),
            Editing(id) => self.view_edit(ctx, id),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        typeset();
    }
}

impl ViewDb {
    fn view_edit(&self, ctx: &Context<Self>, id: usize) -> Html {
        let close_cb = ctx.link().callback(|()| Msg::BackToNormal);
        let edit_cb = ctx.link().callback(Msg::EditInfo);
        let input_data = Rc::new(
            ctx.props()
                .db
                .iter()
                .find(|x| x.id == id)
                .cloned()
                .unwrap_or_else(|| Data::new(id)),
        );
        html! {<EditEntry {close_cb} {edit_cb} {id} {input_data}/>}
    }

    fn view_all(&self, ctx: &Context<Self>) -> Html {
        let filas: Html = self
            .window
            .iter()
            .map(|data| into_row(ctx, data, self.char_length, &self.shown_fields))
            .collect();

        let show_cb: Callback<(bool, Fields)> =
            ctx.link().callback(|(show, field)| Msg::View(show, field));

        let filter_cb: Callback<FilterAction> = ctx.link().callback(Msg::EditFilter);

        let filters: Html = self
            .filters
            .iter()
            .map(|f| {
                html! {
                    <span>{format!("{f} |")}</span>
                }
            })
            .collect();

        let select_cb = ctx.link().callback(Msg::SortField);

        let onchange = ctx.link().callback(Msg::SortAsc);

        let error = self
            .error
            .as_ref()
            .map_or_else(|| html! {}, |err| html! {<p class="error">{err}</p>});

        let range_cb = ctx.link().callback(|(x, y)| Msg::Range(x, y));

        html! {
            <div id="db-table-container">
            {error}
            <ColumnSelect show={self.shown_fields} {show_cb}></ColumnSelect>
            <div>
                <span>{"Ordenar"}</span>
                <FieldSelect {select_cb}/>
                <MatIconButtonToggle {onchange} on={self.sort.ascending} off_icon={Some(AttrValue::Static("⬆️"))} on_icon={Some(AttrValue::Static("⬇️"))}/>
                // <MatIconButtonToggle {onchange}>
                // <MatOnIconButtonToggle>
                // <i class="fa-solid fa-arrow-down-long"></i>
                // </MatOnIconButtonToggle>
                // <MatOffIconButtonToggle>
                // <i class="fa-solid fa-arrow-up-long"></i>
                // </MatOffIconButtonToggle>
                // </MatIconButtonToggle>
                <RangeSelector cb={range_cb} start={self.range.0} end={self.range.1}/>
            </div>
            <FilterAdd {filter_cb}/>
            <div id="filters">{filters}</div>
            <table id="db-table">
                    {header(&self.shown_fields)}
                {filas}
            </table>
            </div>
        }
    }

    fn view_pdf(&self, ctx: &Context<Self>, id: usize) -> Html {
        let return_cb = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::BackToNormal
        });
        let Some(data) = ctx.props().db.get(id) else {
            error!("Pdf {id} is not in db");
        }
        html! {
                <>
                <button onclick={return_cb}>{"Volver"}</button>
        <embed
            src={pdf_url}
            type="application/pdf"
            // width="100%"
            // height="100%"
        />
                </>
            }
    }
}

fn header(shown_fields: &[bool; Fields::N]) -> Html {
    let output = shown_fields
        .iter()
        .zip(Fields::ALL.into_iter())
        .filter_map(|(show, field)| {
            if *show {
                Some(html! {<th>{field.to_string()}</th>})
            } else {
                None
            }
        })
        .collect::<Html>();
    html! {
        <thead><th></th>{output}</thead>
    }
}

fn into_row(
    ctx: &Context<ViewDb>,
    data: &Data,
    max_length: usize,
    shown: &[bool; Fields::N],
) -> Html {
    let entries = shown
        .iter()
        .zip(Fields::ALL.into_iter())
        .filter_map(|(shown, f)| {
            if !shown {
                return None;
            }
            let item = f.get(data).to_owned();
            Some(html! {<FieldDisplay {max_length} {item}   />})
        })
        .collect::<Html>();

    let id = data.id;
    let onclick = ctx.link().callback(move |e: MouseEvent| {
        e.prevent_default();
        Msg::Edit(id)
    });

    let title = data.titulo.clone();
    let delete = ctx.link().callback(move |e: MouseEvent| {
        e.prevent_default();
        Msg::TryDelete {
            id,
            title: title.clone(),
        }
    });

    let bundle = if data.figuras.is_empty() {
        html! {}
    } else if let Some(nombre) = data.tex_url.split('/').last() {
        html! {<a href={AttrValue::from(format!("/PIM/wp-admin/admin-ajax.php?action=paquete_descargar&id={nombre}"))} download={AttrValue::from(format!("{}.zip", data.titulo))}><button class="icon-button" title="Descargar con figuras"><i class="fa-solid fa-file-zipper"></i></button></a>}
    } else {
        log::error!("La url {} está vacía?", data.tex_url);
        html! {}
    };

    let id = data.id;
    let cart = ctx.link().callback(move |e: MouseEvent| {
        e.prevent_default();
        Msg::AddToCart(id)
    });

    html! {
        <tr>
        <td>
        {&data.titulo}
        <a href={data.tex_url.clone()} class="problem-link" title="Descargar .tex">{"tex"}</a>
        <a href={data.pdf_url.clone()} class="problem-link" title="Descargar .pdf">{"pdf"}</a>
        <button title="Editar información" class="edit-button icon-button" {onclick}><i class="fa-solid fa-pen-to-square"></i></button>
        <button class="delete-button icon-button" title="Borrar" onclick={delete}> <i class="fa-solid fa-trash-can"></i></button>
        {bundle}
        <button class="icon-button" title="Añadir al carro" onclick={cart}><i class="fa-solid fa-cart-plus"></i></button>
        </td>
        {entries}
        </tr>
    }
}

impl ViewDb {
    fn calculate_view(&mut self, ctx: &Context<Self>) {
        self.view = ctx
            .props()
            .db
            .iter()
            .cloned()
            .filter(|data| self.filters.iter().all(|filter| filter.passes(data)))
            .collect();
        self.view.sort_by(|a, b| {
            let mut f_a = self.sort.by.get(a);
            let mut f_b = self.sort.by.get(b);
            if !self.sort.ascending {
                (f_b, f_a) = (f_a, f_b);
            }
            f_a.cmp(&f_b)
        });
        self.calculate_window();
    }

    fn calculate_window(&mut self) {
        self.window = self
            .view
            .iter()
            .skip(self.range.0)
            .take(self.range.1.saturating_sub(self.range.0))
            .cloned()
            .collect();
    }

    fn stop_editing(&mut self, ctx: &Context<Self>) {
        self.special_window = Normal;
        let start = self.cached_range.0;
        let end = self.cached_range.1;
        ctx.link().send_future(async move {
            log::info!("Would like to sleep");
            log::info!("Wake up");
            Msg::Range(start, result_range::Which::Start)
        });
        ctx.link()
            .send_future(async move { Msg::Range(end, result_range::Which::End) });
    }
}
