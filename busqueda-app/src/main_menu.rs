use std::path::PathBuf;
use std::rc::Rc;

use crate::commands::insert_db_info;
use crate::handle_db::FetchedData;
use crate::home_button;
use crate::requests::MyRequest;
use pim_lib::Data;
use pim_yew::RawHtml;
use pim_yew::ViewDb as View;
use pim_yew::ViewDbProps;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use AppType::Start;

pub struct MainMenu {
    main_app: AppType,
    db: Option<Rc<Vec<Data>>>,
    error: String,
}

pub enum AppType {
    Start,
    View,
}
pub enum Msg {
    ChangeApps(AppType),
    UpdateDb(Rc<Vec<Data>>),
    UpdateErr(String),
    EditEntry(Data),
    GetDb,
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
        Self::get_db(ctx);
        Self {
            main_app: AppType::Start,
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
            Msg::EditEntry(data) => {
                ctx.link().send_future(async move {
                    let result = insert_db_info(data).await;
                    result.map_or_else(|err| Msg::UpdateErr(err.to_string()), |_| Msg::GetDb)
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let main_app = match self.main_app {
            AppType::Start => Self::view_start(ctx),
            AppType::View => self.view_db(ctx),
        };

        html! {
            <div id="container">
            <RawHtml inner_html={self.error.clone()} tag="div" />
            {main_app}
            </div>
        }
    }
}

impl MainMenu {
    fn get_db(ctx: &Context<Self>) {
        ctx.link().send_future(async move {
            let request = MyRequest::post("/PIM/externos/intranet/problemas-todos.php");
            let response = request.send::<Vec<FetchedData>>().await;

            match response {
                crate::requests::MyResponse::Ok { response } => {
                    let data = response.into_iter().map(Data::from).collect();
                    Msg::UpdateDb(Rc::new(data))
                }
                crate::requests::MyResponse::Code401 => {
                    Msg::UpdateErr("No estás autorizado/a a acceder a la base de datos".into())
                }
                crate::requests::MyResponse::Code500(err) => {
                    Msg::UpdateErr(format!("El servidor ha encontrado un error: {err}"))
                }
                crate::requests::MyResponse::Error(err) => {
                    Msg::UpdateErr(format!("Ha habido un error: {err}"))
                }
            }

            //     let args = serde_wasm_bindgen::to_value(&GetJsonArgs {
            //         json_path: PathTo::Db.default_path(),
            //     })
            //     .expect("Couldn't make into js value🫣");
            //     let db = invoke("get_db_from_json", args).await;
            //     let db: Result<Result<String, String>, _> = serde_wasm_bindgen::from_value(db);
            //     match db {
            //         Ok(Ok(db)) => {
            //             let db = serde_json::from_str(&db);
            //             match db {
            //                 Ok(db) => Msg::UpdateDb(Rc::new(db)),
            //                 Err(err) => {
            //                     Msg::UpdateErr(format!("Error parsing response with serde-json: {err}"))
            //                 }
            //             }
            //         }
            //         Ok(Err(err)) => Msg::UpdateErr(err),
            //         Err(parse_err) => {
            //             Msg::UpdateErr(format!("Error parsing response js value: {parse_err}"))
            //         }
            //     }
        });
    }
    fn view_start(ctx: &Context<Self>) -> Html {
        let view_db = ctx
            .link()
            .callback(|_: MouseEvent| Msg::ChangeApps(AppType::View));
        html! {
            <div id="container">
            <p>{"¿Qué quieres hacer?"}</p>
            <ul>
                <li><button onclick={view_db}>{"Ver la base de datos"}</button></li>
            </ul>
            </div>
        }
    }

    fn view_db(&self, ctx: &Context<Self>) -> Html {
        self.db.as_ref().map_or_else(|| html!{<p>{"Cargando..."}</p>}, |db| {
            let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
            let reload_db_cb = ctx.link().callback(|_| Msg::GetDb);
            let edit_cb = ctx.link().callback( Msg::EditEntry);
            html! {
                <>
                <home_button::With<View> props={ViewDbProps {edit_cb ,db:db.clone(), reload_db_cb}}  {return_cb}></home_button::With<View>>
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
