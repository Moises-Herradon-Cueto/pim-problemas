
use std::path::PathBuf;
use std::rc::Rc;

use crate::app::invoke;
use crate::files_info::{Comp as FilesInfo, PathTo, Paths};
use crate::home_button;
use crate::update_db::{self, UpdateDb as Update};
use pim_lib::Data;
use pim_yew::ViewDb as View;
use pim_yew::ViewDbProps;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use AppType::Start;

pub struct MainMenu {
    main_app: AppType,
    paths: Paths,
    db: Option<Rc<Vec<Data>>>,
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
    UpdateDb(Rc<Vec<Data>>),
    UpdateErr(String),
    EditEntry(Data),
    GetDb,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub paths: Paths,
}
#[derive(Serialize, Deserialize)]
pub struct GetJsonArgs {
    #[serde(rename = "jsonPath")]
    json_path: PathBuf,
}

impl Component for MainMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self::get_db(ctx);
        Self {
            main_app: AppType::Start,
            paths: ctx.props().paths.clone(),
            db: None,
            error: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                self.db = Some(db);
                true
            }
            Msg::UpdateErr(err) => {
                self.error = err;
                true
            }
            Msg::GetDb => {
                Self::get_db(ctx);
                self.db = None;
                false
            }
            Msg::EditEntry(_) => todo!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let paths = self.paths.clone();
        let update_cb = ctx.link().callback(Msg::UpdatePaths);

        let main_app = match self.main_app {
            AppType::Start => Self::view_start(ctx),
            AppType::Update => self.view_update(ctx),
            AppType::View => self.view_db(ctx),
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
    fn get_db(ctx: &Context<Self>) {
        ctx.link().send_future(async move {
            // log::info!("Trying to invoke");
            let args = serde_wasm_bindgen::to_value(&GetJsonArgs {
                json_path: PathTo::Db.default_path(),
            })
            .expect("Couldn't make into js value🫣");
            // log::info!("Made args:\n{args:#?}");
            let db = invoke("get_db_from_json", args).await;
            // log::info!("Created db: {db:#?}");
            let db: Result<Result<String, String>, _> = serde_wasm_bindgen::from_value(db);
            // log::info!("Deserialized DB: {db:?}");
            match db {
                Ok(Ok(db)) => {
                    // log::info!("Received:\n{db}");
                    let db = serde_json::from_str(&db);
                    match db {
                        Ok(db) => Msg::UpdateDb(Rc::new(db)),
                        Err(err) => {
                            Msg::UpdateErr(format!("Error parsing response with serde-json: {err}"))
                        }
                    }
                }
                Ok(Err(err)) => Msg::UpdateErr(err),
                Err(parse_err) => {
                    Msg::UpdateErr(format!("Error parsing response js value: {parse_err}"))
                }
            }
        });
    }
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

    fn view_update(&self, ctx: &Context<Self>) -> Html {
        let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
        let cb = ctx.link().callback(Msg::UpdateDb);
        html! {
            <>
            <home_button::With<Update> props={update_db::Props {paths: self.paths.clone(), cb, db: self.db.clone()}}  {return_cb}>
            </home_button::With<Update>>
            </>
        }
    }

    fn view_db(&self, ctx: &Context<Self>) -> Html {
        self.db.as_ref().map_or_else(|| self.view_update(ctx), |db| {
            let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
            let reload_db_cb = ctx.link().callback(|_| Msg::GetDb);
        let edit_cb = ctx.link().callback(Msg::EditEntry);
            html! {
                <>
                <home_button::With<View> props={ViewDbProps {edit_cb, db:db.clone(),  reload_db_cb}}  {return_cb}></home_button::With<View>>
                </>
            }
        })
    }
}

mod tests {
    use std::collections::HashMap;

    use super::Data;

    pub fn _serialize_deserialize_data() {
        let database = (3_usize..4)
            .map(|x| (x, Data::new(x)))
            .collect::<HashMap<_, _>>();

        log::info!("Before serialization: {database:?}");

        let serialized = serde_wasm_bindgen::to_value(&database).unwrap();

        log::info!("After serialization: {serialized:?}");

        let deserialized = serde_wasm_bindgen::from_value(serialized).unwrap();

        log::info!("After deserialization: {deserialized:?}");

        assert_eq!(database, deserialized);
    }
}
