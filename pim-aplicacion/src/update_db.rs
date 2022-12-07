use std::{path::PathBuf, rc::Rc};

use pim_lib::{Data, Entry, ParseOneError};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::{
    app::invoke,
    files_info::{PathTo, Paths},
};

#[derive(Default)]
pub struct UpdateDb {
    output: Vec<Entry>,
    error: String,
    updated: bool,
}

pub enum Msg {
    ParseFiles,
    UpdateOutput(Vec<Entry>),
    UpdateErr(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub paths: Paths,
    pub cb: Callback<Rc<Vec<Data>>>,
    pub db: Option<Rc<Vec<Data>>>,
}

impl Component for UpdateDb {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ParseFiles => {
                log::info!("Parsing");
                let paths = ctx.props().paths.clone();

                ctx.link().send_future(async move {
                    let parsed = Self::sync_db(paths).await;
                    log::info!("Parsed: {parsed:#?}");
                    match parsed {
                        Ok(Ok(errors)) => Msg::UpdateOutput(errors),
                        Ok(Err(err)) | Err(err) => Msg::UpdateErr(err),
                    }
                });
                self.error = String::from("Cargando...");
                true
            }
            Msg::UpdateErr(error) => {
                self.updated = true;
                self.error = error;
                true
            }
            Msg::UpdateOutput(output) => {
                self.updated = true;
                self.output = output;
                self.error = String::new();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_: MouseEvent| Msg::ParseFiles);
        let list = if self.output.is_empty() && self.updated {
            html! {<p>{"Base de datos actualizada sin errores"}</p>}
        } else {
            self.output.iter().map(show_error).collect::<Html>()
        };
        html! {
            <div>
            <button onclick={onclick}>{"Actualizar"}</button>
            <p>{&self.error}</p>
            <ul>
            {list}
            </ul>

            </div>
        }
    }
}

fn show_error(error: &Entry) -> Html {
    error
        .as_ref()
        .map_or_else(show_parse_error, show_parse_info)
}

fn show_parse_info((id, info): &(usize, pim_lib::ParseOneInfo)) -> Html {
    html! {
        <li class="info">{format!("En el problema {id}: {info}")}</li>
    }
}

fn show_parse_error(err: &ParseOneError) -> Html {
    html! {
        <li class="error">{err}</li>
    }
}

#[derive(Serialize, Deserialize)]
struct UpdateArgs {
    #[serde(rename = "problemsPath")]
    problems_path: PathBuf,
    #[serde(rename = "dbPath")]
    db_path: PathBuf,
    #[serde(rename = "outputPath")]
    output_path: PathBuf,
}
impl UpdateDb {
    #[allow(clippy::future_not_send)]
    async fn sync_db(paths: Paths) -> Result<Result<Vec<Entry>, String>, String> {
        let problems_path = paths.get(PathTo::Problems).into_owned();
        let db_path = paths.get(PathTo::Db).into_owned();
        let output_path = paths.get(PathTo::Output).into_owned();
        let args = serde_wasm_bindgen::to_value(&UpdateArgs {
            problems_path,
            db_path,
            output_path,
        })
        .expect("Couldn't make into js value🫣");
        let invoke_result = invoke("update_db", args).await;
        let parsed_result: Result<String, String> =
            serde_wasm_bindgen::from_value(invoke_result)
                .map_err(|err| format!("Error converting js value to value: {err}"))?;
        let value = parsed_result?;
        log::info!("About to deserialize: {value:#}");
        serde_json::from_str(&value).map_err(|err| format!("Error deserializing {err}"))
    }
}
