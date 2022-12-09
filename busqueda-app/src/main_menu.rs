use std::path::PathBuf;
use std::rc::Rc;

use crate::commands::delete;
use crate::commands::insert_db_info;
use crate::handle_db::FetchedData;
use crate::home_button;
use crate::requests::MyRequest;
use pim_lib::Data;
use pim_yew::Cart;
use pim_yew::Direction;
use pim_yew::RawHtml;
use pim_yew::ViewDb as View;
use pim_yew::ViewDbProps;
use serde::{Deserialize, Serialize};
use web_sys::window;
use web_sys::Storage;
use yew::prelude::*;
use AppType::Start;

pub struct MainMenu {
    main_app: AppType,
    db: Option<Rc<Vec<Data>>>,
    cart: Vec<usize>,
    error: String,
}

pub enum AppType {
    Start,
    View,
}
#[allow(clippy::large_enum_variant)]
pub enum Msg {
    ChangeApps(AppType),
    UpdateDb(Rc<Vec<Data>>),
    UpdateErr(String),
    EditEntry(Data),
    DeleteProblem(usize),
    AddToCart(usize),
    GetDb,
    GetCart,
    ReorderCartWithIndex(usize, Direction),
    RemoveIndexFromCart(usize),
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
        let cart = get_cart_from_storage().unwrap_or_default();
        Self {
            main_app: AppType::Start,
            db: None,
            cart,
            error: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetCart => {
                log::info!("Get cart!");
                false
            }
            Msg::ReorderCartWithIndex(index, dir) => {
                match dir {
                    Direction::Up => {
                        if index == 0 {
                            return false;
                        }
                        self.cart.swap(index, index - 1);
                    }
                    Direction::Down => {
                        if index >= self.cart.len() - 1 {
                            return false;
                        }
                        self.cart.swap(index, index + 1);
                    }
                }
                true
            }
            Msg::RemoveIndexFromCart(index) => {
                if index >= self.cart.len() {
                    return false;
                }
                self.cart.remove(index);
                true
            }
            Msg::AddToCart(id) => {
                if self.cart.last() == Some(&id) {
                    return false;
                }
                self.cart.push(id);
                append_to_cart_storage(id);
                true
            }
            Msg::DeleteProblem(id) => {
                ctx.link().send_future(async move {
                    let delete = delete(id).await;
                    delete.map_or_else(Msg::UpdateErr, |_| Msg::GetDb)
                });
                false
            }
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
                    result.map_or_else(Msg::UpdateErr, |_| Msg::GetDb)
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
            if !self.cart.is_empty() {
                {self.carro_html(ctx)}
            }
            {main_app}
            </div>
        }
    }
}

impl MainMenu {
    fn carro_html(&self, ctx: &Context<Self>) -> Html {
        let Some(db) = self.db.as_ref() else {return html!{}};
        let db = Rc::clone(db);
        let download = ctx.link().callback(|_| Msg::GetCart);
        let list = self.cart.clone();
        let move_up = ctx
            .link()
            .callback(|(index, dir)| Msg::ReorderCartWithIndex(index, dir));
        let remove_index = ctx.link().callback(Msg::RemoveIndexFromCart);
        html! {
            <Cart {list} {db} {remove_index} {download} {move_up}/>
        }
    }
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
            let delete_cb = ctx.link().callback(Msg::DeleteProblem);
            let add_to_cart = ctx.link().callback(Msg::AddToCart);
            html! {
                <>
                <home_button::With<View> props={ViewDbProps {add_to_cart, delete_cb, edit_cb ,db:db.clone(), reload_db_cb}}  {return_cb}></home_button::With<View>>
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

fn storage() -> Option<Storage> {
    window()?.local_storage().unwrap_or_default()
}

fn get_cart_from_storage() -> Option<Vec<usize>> {
    let storage = storage()?;
    let fields = storage.get("carrito").unwrap_or_default()?;
    Some(
        fields
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect(),
    )
}

fn append_to_cart_storage(id: usize) {
    let mut cart = get_cart_from_storage().unwrap_or_default();
    cart.push(id);
    let Some(storage) =storage() else {
        log::error!("Failed to access local storage");
        return;
    };
    let cart: Vec<String> = cart.into_iter().map(|x| x.to_string()).collect();
    storage
        .set_item("carrito", &cart.join(","))
        .unwrap_or_else(|err| {
            log::error!("Error escribiendo en el almacenamiento local:\n{err:?}");
        });
}
