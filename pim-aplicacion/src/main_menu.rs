use crate::files_info::{Comp as FilesInfo, Paths};
use crate::home_button;
use crate::update_db::UpdateDb as Update;
use crate::view_db::ViewDb as View;
use yew::prelude::*;
use AppType::Start;

pub struct MainMenu {
    main_app: AppType,
    paths: Paths,
}

pub enum AppType {
    Start,
    Update,
    View,
}
pub enum Msg {
    ChangeApps(AppType),
    UpdatePaths(Paths),
}

impl Component for MainMenu {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            main_app: AppType::Start,
            paths: Paths::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeApps(app) => {
                self.main_app = app;
                true
            }
            Msg::UpdatePaths(paths) => {
                self.paths = paths;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let paths = self.paths.clone();
        let update_cb = ctx.link().callback(Msg::UpdatePaths);

        let main_app = match self.main_app {
            AppType::Start => Self::view_start(ctx),
            AppType::Update => Self::view_update(ctx),
            AppType::View => Self::view_db(ctx),
        };

        html! {
            <div id="container">
            <p>{format!("Archivos: {:?}, {:?}", self.paths.problems, self.paths.database)}</p>
            <FilesInfo {paths} {update_cb}></FilesInfo>
            {main_app}
            </div>
        }
    }
}

impl MainMenu {
    fn view_start(ctx: &Context<Self>) -> Html {
        let update_db = ctx
            .link()
            .callback(|_: MouseEvent| Msg::ChangeApps(AppType::Update));
        let view_db = ctx
            .link()
            .callback(|_: MouseEvent| Msg::ChangeApps(AppType::View));
        html! {
            <div id="container">
            <p>{"¿Qué quieres hacer?"}</p>
            <ul>
                <li><button onclick={update_db}>{"Actualizar la base de datos"}</button></li>
                <li><button onclick={view_db}>{"Ver la base de datos"}</button></li>
            </ul>
            </div>
        }
    }

    fn view_update(ctx: &Context<Self>) -> Html {
        let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
        html! {
            <>
            <home_button::With<Update> props={()} {return_cb}></home_button::With<Update>>
            </>
        }
    }

    fn view_db(ctx: &Context<Self>) -> Html {
        let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
        html! {
            <>
            <home_button::With<View> props={()}  {return_cb}></home_button::With<View>>
            </>
        }
    }
}
