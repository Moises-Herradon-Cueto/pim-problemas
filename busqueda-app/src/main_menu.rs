use std::path::PathBuf;
use std::rc::Rc;

use crate::commands::delete;
use crate::commands::insert_db_info;
use crate::handle_db::FetchedData;
use crate::helper::undo_waiting_cursor;
use crate::helper::waiting_cursor;
use crate::helper::IndexOf;
use crate::home_button;
use crate::requests::MyRequest;
use pim_lib::Data;
use pim_yew::{
    get_sheet, Cart, Direction, RawHtml, Sheet, SheetEditor, SheetEditorProps, Sheets, SheetsProps,
    ViewDb as View, ViewDbProps,
};
use serde::{Deserialize, Serialize};
use web_sys::window;
use web_sys::Storage;
use yew::prelude::*;
use AppType::Start;

pub struct MainMenu {
    main_app: AppType,
    db: Option<Rc<Vec<Data>>>,
    sheets: Option<Vec<Sheet>>,
    real_cart: Vec<usize>,
    cart_clone: Rc<Vec<usize>>,
    error: String,
}

pub enum AppType {
    Start,
    View,
    Sheets,
    SheetEditor,
}
#[allow(clippy::large_enum_variant)]
pub enum Msg {
    ChangeApps(AppType),
    UpdateDb(Rc<Vec<Data>>),
    UpdateSheets(Vec<Sheet>),
    UpdateErr(String),
    EditEntry(Data),
    DeleteProblem(usize),
    ToggleCart(usize),
    GetDb,
    GetSheets,
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
        waiting_cursor();
        Self::get_db(ctx);
        Self::get_sheets(ctx);
        let real_cart = get_cart_from_storage().unwrap_or_default();
        let cart_clone = Rc::new(real_cart.clone());
        Self {
            main_app: AppType::Start,
            db: None,
            sheets: None,
            real_cart,
            cart_clone,
            error: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetCart => {
                let list: Vec<_> = self
                    .cart_clone
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                get_sheet(list.join(","));
                false
            }
            Msg::ReorderCartWithIndex(index, dir) => {
                match dir {
                    Direction::Up => {
                        if index == 0 {
                            return false;
                        }
                        self.real_cart.swap(index, index - 1);
                    }
                    Direction::Down => {
                        if index >= self.cart_clone.len() - 1 {
                            return false;
                        }
                        self.real_cart.swap(index, index + 1);
                    }
                }
                set_cart(&self.real_cart);
                self.cart_clone = Rc::new(self.real_cart.clone());
                true
            }
            Msg::RemoveIndexFromCart(index) => {
                if index >= self.cart_clone.len() {
                    return false;
                }
                self.real_cart.remove(index);
                set_cart(&self.real_cart);
                self.cart_clone = Rc::new(self.real_cart.clone());
                true
            }
            Msg::ToggleCart(id) => {
                if let Some(index) = self.cart_clone.index_of(&id) {
                    self.real_cart.remove(index);
                    remove_index_from_cart_storage(index);
                } else {
                    self.real_cart.push(id);
                    append_to_cart_storage(id);
                }
                self.cart_clone = Rc::new(self.real_cart.clone());
                true
            }
            Msg::DeleteProblem(id) => {
                waiting_cursor();
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
                undo_waiting_cursor();
                self.db = Some(db);
                true
            }
            Msg::UpdateSheets(sheets) => {
                undo_waiting_cursor();
                self.sheets = Some(sheets);
                true
            }
            Msg::UpdateErr(err) => {
                undo_waiting_cursor();
                self.error = err;
                true
            }
            Msg::GetDb => {
                waiting_cursor();
                Self::get_db(ctx);
                false
            }
            Msg::GetSheets => {
                waiting_cursor();
                Self::get_sheets(ctx);
                false
            }
            Msg::EditEntry(data) => {
                waiting_cursor();
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
            AppType::Sheets => self.view_sheets(ctx),
            AppType::SheetEditor => self.sheet_editor(ctx),
        };

        html! {
            <div id="container">
            <RawHtml inner_html={self.error.clone()} tag="div" />
            if !self.cart_clone.is_empty() {
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
        let list = Rc::clone(&self.cart_clone);
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
            let request = MyRequest::post("/PIM/wp-admin/admin-ajax.php?action=problemas_todos");
            let response = request.send::<Vec<FetchedData>>().await;

            match response {
                crate::requests::MyResponse::Ok { response } => {
                    let data = response.into_iter().map(Data::from).collect();
                    Msg::UpdateDb(Rc::new(data))
                }
                crate::requests::MyResponse::Code401 => {
                    Msg::UpdateErr("No estás autorizado/a a acceder a la base de datos".into())
                }
                crate::requests::MyResponse::Code500(err) => Msg::UpdateErr(format!(
                    "El servidor ha encontrado un error procesando los problemas: {err}"
                )),
                crate::requests::MyResponse::Error(err) => Msg::UpdateErr(format!(
                    "Ha habido un error procesando los problemas: {err}"
                )),
            }
        });
    }

    fn get_sheets(ctx: &Context<Self>) {
        ctx.link().send_future(async move {
            let request = MyRequest::post("/PIM/wp-admin/admin-ajax.php?action=hojas_todas");
            let response = request.send::<Vec<Sheet>>().await;

            match response {
                crate::requests::MyResponse::Ok { response } => Msg::UpdateSheets(response),
                crate::requests::MyResponse::Code401 => {
                    Msg::UpdateErr("No estás autorizado/a a acceder a la base de datos".into())
                }
                crate::requests::MyResponse::Code500(err) => Msg::UpdateErr(format!(
                    "El servidor ha encontrado un error procesando las hojas: {err}"
                )),
                crate::requests::MyResponse::Error(err) => {
                    Msg::UpdateErr(format!("Ha habido un error procesando las hojas: {err}"))
                }
            }
        });
    }
    fn view_start(ctx: &Context<Self>) -> Html {
        let view_sheets = ctx
            .link()
            .callback(|_: MouseEvent| Msg::ChangeApps(AppType::Sheets));
        let view_db = ctx
            .link()
            .callback(|_: MouseEvent| Msg::ChangeApps(AppType::View));
        let view_editor = ctx
            .link()
            .callback(|_: MouseEvent| Msg::ChangeApps(AppType::SheetEditor));
        html! {
            <div id="container">
            <p>{"¿Qué quieres hacer?"}</p>
            <ul>
                <li><button onclick={view_db}>{"Ver la base de datos y editar la información"}</button></li>
                <li><button onclick={view_sheets}>{"Ver las hojas (aún no se puede hacer mucho con ellas)"}</button></li>
                <li><button onclick={view_editor}>{"Crear hojas a partir del carro"}</button></li>
            </ul>
            </div>
        }
    }

    fn view_db(&self, ctx: &Context<Self>) -> Html {
        self.db.as_ref().map_or_else(|| html!{<p>{"Cargando..."}</p>}, |db| {
            let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
            let reload_db_cb = ctx.link().batch_callback(|_| vec![Msg::GetDb,Msg::GetSheets ]);
            let edit_cb = ctx.link().callback( Msg::EditEntry);
            let delete_cb = ctx.link().callback(Msg::DeleteProblem);
            let toggle_cart = ctx.link().callback(Msg::ToggleCart);
            html! {
                <>
                <home_button::With<View> props={ViewDbProps {toggle_cart, delete_cb, edit_cb ,db:db.clone(), reload_db_cb, cart: Rc::clone(&self.cart_clone)}}  {return_cb}></home_button::With<View>>
                </>
            }
        })
    }

    fn view_sheets(&self, ctx: &Context<Self>) -> Html {
        let loading = html! {
            <p>{"Cargando..."}</p>
        };
        let Some(db) = self.db.as_ref() else {return loading;};
        let Some(sheets) = self.sheets.as_ref() else {return loading;};
        let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
        let reload_sheets_cb = ctx.link().callback(|_| Msg::GetSheets);
        html! {
            <home_button::With<Sheets> props={SheetsProps {reload_sheets_cb, db: db.clone(), sheets: sheets.clone()}} {return_cb}/>
        }
    }

    fn sheet_editor(&self, ctx: &Context<Self>) -> Html {
        let Some(db) = self.db.as_ref() else {return html! {
            <p>{"Cargando..."}</p>
        };};
        let return_cb = ctx.link().callback(|_: ()| Msg::ChangeApps(Start));
        html! {
            <home_button::With<SheetEditor> {return_cb} props={SheetEditorProps {cart: Rc::clone(&self.cart_clone), db: Rc::clone(db)}}/>
        }
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
    if fields.trim().is_empty() {
        return None;
    }
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
    set_cart(&cart);
}
fn remove_index_from_cart_storage(index: usize) {
    let mut cart = get_cart_from_storage().unwrap_or_default();
    cart.remove(index);
    set_cart(&cart);
}

fn set_cart(cart: &[usize]) {
    let Some(storage) =storage() else {
        log::error!("Failed to access local storage");
        return;
    };
    let cart: Vec<String> = cart.iter().map(std::string::ToString::to_string).collect();
    storage
        .set_item("carrito", &cart.join(","))
        .unwrap_or_else(|err| {
            log::error!("Error escribiendo en el almacenamiento local:\n{err:?}");
        });
}
