use std::rc::Rc;

use crate::field_edit_entry::Comp as FieldEntryEdit;
use parse_lib::{Data, FieldContents, Fields};
use yew::prelude::*;

#[derive(Default)]
pub enum Comp {
    #[default]
    Closed,
    Open(Data),
}

pub enum Msg {
    Open,
    Close,
    Edit(FieldContents),
    Submit,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: usize,
    pub edit_cb: Callback<Data>,
    pub input_data: Rc<Data>,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::Closed
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                *self = Self::Closed;
                true
            }
            Msg::Open => {
                let data = ctx.props().input_data.clone();
                *self = Self::Open((*data).clone());
                true
            }
            Msg::Edit(content) => {
                if let Self::Open(data) = self {
                    data.set(content);
                }
                false
            }
            Msg::Submit => {
                if let Self::Open(_) = self {
                    let data = std::mem::take(self);
                    if let Self::Open(data) = data {
                        ctx.props().edit_cb.emit(data);
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self {
            Self::Closed => {
                let onclick = ctx.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    Msg::Open
                });
                html! {<td><button class="edit-button" {onclick}><i class="fa-solid fa-pen-to-square"></i></button></td>}
            }
            Self::Open(data) => {
                let edit_cb = ctx.link().callback(Msg::Edit);
                let rows: Html = Fields::ALL.into_iter().map(|field| {
                    html!{
                        <FieldEntryEdit edit_cb = {edit_cb.clone()} contents = {field.get(data).to_owned()}/>
                    }
                }).collect();
                let close_cb = ctx.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    Msg::Close
                });
                let submit_cb = ctx.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    Msg::Submit
                });

                html! {
                    <td>
                    <div class="edit-problem">
                    <form>
                    <fieldset>
                        <button class="close-edit-button icon-button" onclick={close_cb}>
                        <i class="fa-solid fa-xmark"></i>
                        </button>
                            {rows}
                        <button class="submit-edit-button" onclick={submit_cb}>{"Aceptar"}</button>
                    </fieldset>
                    </form>
                    </div>
                    </td>
                }
            }
        }
    }
}
