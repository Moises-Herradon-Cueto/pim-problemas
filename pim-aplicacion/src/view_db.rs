use std::{collections::HashMap, rc::Rc};

use crate::column_select::Comp as ColumnSelect;
use parse_lib::{Data, Fields};
use yew::prelude::*;

pub struct ViewDb {
    view: Vec<Data>,
    shown_fields: [bool; Fields::N],
    char_length: usize,
}

pub enum Msg {
    View(bool, Fields),
}

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct Props {
    pub db: Rc<HashMap<usize, Data>>,
}

impl Component for ViewDb {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut view: Vec<_> = ctx
            .props()
            .db
            .iter()
            .map(|(_, problem_info)| problem_info.clone())
            .collect();
        view.sort_by(|a, b| a.id.cmp(&b.id));
        let mut shown_fields = [true; Fields::N];
        for (i, f) in Fields::ALL.into_iter().enumerate() {
            shown_fields[i] = f.is_in_template();
        }
        Self {
            view,
            shown_fields,
            char_length: 100,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::View(show, field) => {
                self.shown_fields[field as usize] = show;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let filas: Html = self
            .view
            .iter()
            .map(|data| into_row(data, self.char_length, &self.shown_fields))
            .collect();

        let show_cb: Callback<(bool, Fields)> =
            ctx.link().callback(|(show, field)| Msg::View(show, field));

        html! {
            <div id="db-table-container">
            <ColumnSelect show={self.shown_fields} {show_cb}></ColumnSelect>
            <table id="db-table">
                    {header(&self.shown_fields)}
                {filas}
            </table>
            </div>
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
        <thead>{output}</thead>
    }
}

fn into_row(data: &Data, max_length: usize, shown: &[bool; Fields::N]) -> Html {
    let entries = shown
        .iter()
        .zip(Fields::ALL.into_iter())
        .filter_map(|(shown, f)| {
            if !shown {
                return None;
            }
            let msg = f.get_string(data);
            let string = msg.chars().take(max_length).collect::<String>();
            Some(html! {<td>{string}</td>})
        })
        .collect::<Html>();

    html! {
        <tr>
        {entries}
        </tr>
    }
}
