use crate::files_info::Comp as FilesInfo;
use crate::home_button;
use crate::update_db::UpdateDb as Update;
use crate::view_db::ViewDb as View;
use yew::prelude::*;

pub enum MainMenu {
    Start,
    Update,
    View,
}

pub enum Msg {
    UpdateDb,
    ViewDb,
    ToStart,
}

impl From<Msg> for MainMenu {
    fn from(msg: Msg) -> Self {
        match msg {
            Msg::UpdateDb => Self::Update,
            Msg::ViewDb => Self::View,
            Msg::ToStart => Self::Start,
        }
    }
}

impl Component for MainMenu {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::Start
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        *self = Self::from(msg);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self {
            Self::Start => Self::view_start(ctx),
            Self::Update => Self::view_update(ctx),
            Self::View => Self::view_db(ctx),
        }
    }
}

impl MainMenu {
    fn view_start(ctx: &Context<Self>) -> Html {
        let update_db = ctx.link().callback(|_: MouseEvent| Msg::UpdateDb);
        let view_db = ctx.link().callback(|_: MouseEvent| Msg::ViewDb);
        html! {
            <div id="container">
            <h1>{"OLA"}</h1>
            <FilesInfo></FilesInfo>
            <p>{"¿Qué quieres hacer?"}</p>
            <ul>
                <li><button onclick={update_db}>{"Actualizar la base de datos"}</button></li>
                <li><button onclick={view_db}>{"Ver la base de datos"}</button></li>
            </ul>
            </div>
        }
    }

    fn view_update(ctx: &Context<Self>) -> Html {
        let return_cb = ctx.link().callback(|_: ()| Msg::ToStart);
        html! {
            <>
            <FilesInfo></FilesInfo>
            <home_button::With<Update> props={()} {return_cb}></home_button::With<Update>>
            </>
        }
    }

    fn view_db(ctx: &Context<Self>) -> Html {
        let return_cb = ctx.link().callback(|_: ()| Msg::ToStart);
        html! {
            <>
            <FilesInfo></FilesInfo>
            <home_button::With<View> props={()}  {return_cb}></home_button::With<View>>
            </>
        }
    }
}
