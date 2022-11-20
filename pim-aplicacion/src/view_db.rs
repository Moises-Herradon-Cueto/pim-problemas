use std::{collections::HashMap, rc::Rc};

use crate::add_filters::{Comp as FilterAdd, Filter, FilterAction};
use crate::app::typeset;
use crate::column_select::Comp as ColumnSelect;
use crate::commands::insert_db_info;
use crate::edit_entry::Comp as EditEntry;
use crate::field_display::Comp as FieldDisplay;
use crate::field_selector::Comp as FieldSelect;
use crate::files_info::{PathTo, Paths};
use material_yew::MatIconButtonToggle;
use parse_lib::{Data, Fields, ParseOneError};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

pub struct ViewDb {
    view: Vec<Data>,
    shown_fields: [bool; Fields::N],
    char_length: usize,
    filters: Vec<Filter>,
    sort: Sort,
    error: Option<ParseOneError>,
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

pub enum Msg {
    View(bool, Fields),
    EditFilter(FilterAction),
    SortField(Fields),
    SortAsc(bool),
    EditInfo(Data),
    SetError(ParseOneError),
    ReloadDb,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub db: Rc<HashMap<usize, Data>>,
    pub reload_db_cb: Callback<()>,
    pub paths: Paths,
}

impl Component for ViewDb {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut shown_fields = [true; Fields::N];
        for (i, f) in Fields::ALL.into_iter().enumerate() {
            shown_fields[i] = f.is_in_template();
        }
        let mut output = Self {
            view: vec![],
            shown_fields,
            char_length: 100,
            filters: vec![],
            sort: Sort::default(),
            error: None,
        };

        output.calculate_view(ctx);

        output
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetError(err) => {
                self.error = Some(err);
            }
            Msg::ReloadDb => {
                ctx.props().reload_db_cb.emit(());
                return false;
            }
            Msg::EditInfo(data) => {
                let problems_path = ctx.props().paths.get(PathTo::Problems).into_owned();
                let db_path = ctx.props().paths.get(PathTo::Db).into_owned();
                ctx.link().send_future(async move {
                    let result = insert_db_info(&problems_path, &db_path, data).await;
                    result.map_or_else(Msg::SetError, |_| Msg::ReloadDb)
                });
                return false;
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
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let edit_cb = ctx.link().callback(Msg::EditInfo);
        let filas: Html = self
            .view
            .iter()
            .map(|data| {
                into_row(
                    Rc::new(data.clone()),
                    self.char_length,
                    &self.shown_fields,
                    edit_cb.clone(),
                )
            })
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

        html! {
            <div id="db-table-container">
            {error}
            <ColumnSelect show={self.shown_fields} {show_cb}></ColumnSelect>
            <div>
                <span>{"Ordenar"}</span>
                <FieldSelect {select_cb}/>
                <MatIconButtonToggle {onchange} off_icon={Some(AttrValue::Static("⬆️"))} on_icon={Some(AttrValue::Static("⬇️"))}/>
                // <MatIconButtonToggle {onchange}>
                // <MatOnIconButtonToggle>
                // <i class="fa-solid fa-arrow-down-long"></i>
                // </MatOnIconButtonToggle>
                // <MatOffIconButtonToggle>
                // <i class="fa-solid fa-arrow-up-long"></i>
                // </MatOffIconButtonToggle>
                // </MatIconButtonToggle>
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

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        typeset();
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
    data: Rc<Data>,
    max_length: usize,
    shown: &[bool; Fields::N],
    edit_cb: Callback<Data>,
) -> Html {
    let entries = shown
        .iter()
        .zip(Fields::ALL.into_iter())
        .filter_map(|(shown, f)| {
            if !shown {
                return None;
            }
            let item = f.get(&data).to_owned();
            Some(html! {<FieldDisplay {max_length} {item}   />})
        })
        .collect::<Html>();

    html! {
        <tr>
        <EditEntry {edit_cb} id={data.id} input_data={data}/>
        {entries}
        </tr>
    }
}

impl ViewDb {
    fn calculate_view(&mut self, ctx: &Context<Self>) {
        let mut view: Vec<_> = ctx
            .props()
            .db
            .iter()
            .map(|(_, problem_info)| problem_info.clone())
            .filter(|data| self.filters.iter().all(|filter| filter.passes(data)))
            .collect();
        view.sort_by(|a, b| {
            let mut f_a = self.sort.by.get(a);
            let mut f_b = self.sort.by.get(b);
            if !self.sort.ascending {
                (f_b, f_a) = (f_a, f_b);
            }
            f_a.cmp(&f_b)
        });
        self.view = view.into_iter().take(5).collect();
    }
}
