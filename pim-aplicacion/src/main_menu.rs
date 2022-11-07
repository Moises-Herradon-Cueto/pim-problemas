use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::app::invoke;
use crate::files_info::{Comp as FilesInfo, Paths, DEFAULT_DB};
use crate::update_db::UpdateDb as Update;
use crate::view_db::ViewDb as View;
use crate::{home_button, DB};
use parse_lib::data::Data;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use yew::prelude::*;
use AppType::Start;

pub struct MainMenu {
    main_app: AppType,
    paths: Paths,
    db: Option<Rc<HashMap<usize, Data>>>,
    error: String,
}

pub enum AppType {
    Start,
    Update,
    View,
}
pub enum Msg {
    ChangeApps(AppType),
    UpdatePaths(Paths),
    UpdateDb(HashMap<usize, Data>),
    UpdateErr(String),
}

#[derive(Serialize, Deserialize)]
pub struct GetJsonArgs {
    #[serde(rename = "jsonPath")]
    json_path: PathBuf,
}

impl Component for MainMenu {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let db = invoke(
                "get_db_from_json",
                to_value(&GetJsonArgs {
                    json_path: PathBuf::from(DEFAULT_DB),
                })
                .unwrap(),
            )
            .await;
            let db: Result<Result<DB, String>, _> = serde_wasm_bindgen::from_value(db);
            match db {
                Ok(Ok(db)) => Msg::UpdateDb(db),
                Ok(Err(err)) => Msg::UpdateErr(err),
                Err(parse_err) => Msg::UpdateErr(format!("Error parsing response: {parse_err}")),
            }
        });
        Self {
            main_app: AppType::Start,
            paths: Paths::default(),
            db: None,
            error: String::new(),
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
            Msg::UpdateDb(db) => {
                log::info!("{db:?}");
                self.db = Some(Rc::new(db));
                false
            }
            Msg::UpdateErr(err) => {
                self.error = err;
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
            <p>{&self.error}</p>
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
