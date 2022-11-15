use parse_lib::{Enunciado, FieldContents};
use yew::prelude::*;

use crate::raw_html;

pub struct Comp;

pub enum Msg {}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub max_length: usize,
    pub item: FieldContents,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().item {
            // FieldContents::Problem(Enunciado { raw: _, html }) => {
            //     html! {
            //         <raw_html::Comp tag={String::from("td")} inner_html={html.clone()}/>
            //     }
            // }
            x => {
                let x = x.string_contents();
                html! {<td>{x}</td>}
            }
        }
    }
}
