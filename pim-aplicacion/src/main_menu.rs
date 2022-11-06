use crate::update::Update;
use yew::prelude::*;

pub enum MainMenu {
    Start,
    Update,
    View,
}

pub enum MainMenuMsg {
    UpdateDb,
    ViewDb,
    ToStart,
}

impl From<MainMenuMsg> for MainMenu {
    fn from(msg: MainMenuMsg) -> Self {
        match msg {
            MainMenuMsg::UpdateDb => Self::Update,
            MainMenuMsg::ViewDb => Self::View,
            MainMenuMsg::ToStart => Self::Start,
        }
    }
}

impl Component for MainMenu {
    type Message = MainMenuMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::Start
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        *self = MainMenu::from(msg);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self {
            MainMenu::Start => self.view_start(ctx),
            MainMenu::Update => self.view_update(ctx),
            MainMenu::View => self.view_db(ctx),
        }
    }
}

impl MainMenu {
    fn view_start(&self, ctx: &Context<Self>) -> Html {
        let update_db = ctx.link().callback(|_: MouseEvent| MainMenuMsg::UpdateDb);
        let view_db = ctx.link().callback(|_: MouseEvent| MainMenuMsg::ViewDb);
        html! {
            <div id="container">
            <h1>{"OLA"}</h1>
            <p>{"¿Qué quieres hacer?"}</p>
            <ul>
                <li><button onclick={update_db}>{"Actualizar la base de datos"}</button></li>
                <li><button onclick={view_db}>{"Ver la base de datos"}</button></li>
            </ul>
            </div>
        }
    }

    fn view_update(&self, ctx: &Context<Self>) -> Html {
        let return_cb = ctx.link().callback(|_: ()| MainMenuMsg::ToStart);
        html! {
            <Update {return_cb}></Update>
        }
    }

    fn view_db(&self, ctx: &Context<Self>) -> Html {
        let return_cb = ctx.link().callback(|_: ()| MainMenuMsg::ToStart);
        html! {
            <Update {return_cb}></Update>
        }
    }
}
