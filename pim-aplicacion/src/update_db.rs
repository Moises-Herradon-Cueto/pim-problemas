use yew::prelude::*;

use crate::files_info::Paths;

pub struct UpdateDb;

pub enum Msg {
    ParseFiles,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    paths: Paths,
}

impl Component for UpdateDb {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ParseFiles => {
                log::info!("Unimplemented");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_: MouseEvent| Msg::ParseFiles);
        html! {
            <div>
            <button onclick={onclick}>{"Actualizar"}</button>

            </div>
        }
    }
}
