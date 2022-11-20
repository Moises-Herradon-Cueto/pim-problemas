use yew::prelude::*;

use crate::files_info::Comp as FilesInfo;
use crate::files_info::Paths;
use crate::main_menu::MainMenu;

pub struct Comp {
    done: bool,
    paths: Paths,
}

pub enum Msg {
    UpdatePaths(Paths),
    Done,
}
#[derive(PartialEq, Eq, Clone, Properties)]
pub struct Props {}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            done: false,
            paths: Paths::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdatePaths(paths) => {
                self.paths = paths;
            }
            Msg::Done => {
                self.done = true;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.done {
            html! {<MainMenu paths={self.paths.clone()}/>}
        } else {
            let update_cb = ctx.link().callback(Msg::UpdatePaths);
            let onclick = ctx.link().callback(|e: MouseEvent| {
                e.prevent_default();
                Msg::Done
            });
            html! {
                <>
                <h1>{"Gestionar los problemas del PIM"}</h1>
                <p>{"Elige dónde están los problemas y dónde se tienen que escribir documentos .tex modificados"}</p>
                <FilesInfo {update_cb} paths = {self.paths.clone()}/>
                <button {onclick}>{"Aceptar"}</button>
                </>
            }
        }
    }
}
