use std::rc::Rc;

use crate::FieldEditEntry;
use pim_lib::{Data, FieldContents, Fields};
use yew::{prelude::*, virtual_dom::AttrValue};

pub struct Comp {
    data: Data,
}

pub enum Msg {
    Edit(FieldContents),
    Close,
    Submit,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: usize,
    pub edit_cb: Callback<Data>,
    pub close_cb: Callback<()>,
    pub input_data: Rc<Data>,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: (*ctx.props().input_data).clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Edit(content) => {
                self.data.set(content);
            }
            Msg::Submit => {
                ctx.props().edit_cb.emit(self.data.clone());
            }
            Msg::Close => {
                ctx.props().close_cb.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let edit_cb = ctx.link().callback(Msg::Edit);
        let rows: Html = Fields::ALL.into_iter().map(|field| {
                    if matches!(field, Fields::Problem) {return html!{};}
                    html!{
                        <FieldEditEntry edit_cb = {edit_cb.clone()} contents = {field.get(&self.data).to_owned()}/>
                    }
                }).collect();
        let submit_cb = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::Submit
        });
        let close_cb = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::Close
        });

        html! {
            <div class="edit-problem">
            <form>
            <fieldset>
                <button class="close-edit-button icon-button" onclick={close_cb}>
                <i class="fa-solid fa-xmark"></i>
                </button>
                    <div class="two-columns">
                        <div>
                        <button><a href={AttrValue::from(format!("/PIM/externos/intranet/problemas-anadir.php?edit={}", self.data.id))} target="_blank">
                        {"Volver a subir el problema"}
                        </a></button>
                        {rows}
                        </div>
                        <div>
                        <FieldEditEntry edit_cb = {edit_cb.clone()} contents = {Fields::Problem.get(&self.data).to_owned()}/>
                        </div>
                    </div>
                <button class="submit-edit-button" onclick={submit_cb}>{"Aceptar"}</button>
            </fieldset>
            </form>
            </div>
        }
    }
}
