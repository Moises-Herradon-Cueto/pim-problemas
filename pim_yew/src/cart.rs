use std::{collections::HashSet, rc::Rc};

use pim_lib::Data;
use yew::{prelude::*, virtual_dom::AttrValue};

use crate::callbacks::button_callback;

pub struct Comp {
    showing: HashSet<usize>, //indices of the ones we are showing
    open: bool,
}

pub enum Msg {
    ToggleShow(usize),
    Download,
    RemoveIndex(usize),
    MoveUp(usize),
    MoveDown(usize),
    Open,
    Close,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub list: Vec<usize>,
    pub db: Rc<Vec<Data>>,
    pub remove_index: Callback<usize>,
    pub download: Callback<()>,
    pub move_up: Callback<(usize, Direction)>,
}

pub enum Direction {
    Up,
    Down,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            open: false,
            showing: HashSet::new(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleShow(i) => {
                if !self.showing.insert(i) {
                    self.showing.remove(&i);
                }
                true
            }
            Msg::Download => {
                ctx.props().download.emit(());
                false
            }
            Msg::RemoveIndex(id) => {
                ctx.props().remove_index.emit(id);
                false
            }
            Msg::Open => {
                self.open = true;
                true
            }
            Msg::Close => {
                self.open = false;
                true
            }
            Msg::MoveUp(index) => {
                ctx.props().move_up.emit((index, Direction::Up));
                false
            }
            Msg::MoveDown(index) => {
                ctx.props().move_up.emit((index, Direction::Down));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.open {
            let onclick = ctx.link().callback(|e: MouseEvent| {
                e.prevent_default();
                Msg::Open
            });
            return html! {
                <button {onclick} id="cart-button"><i class="fa-solid fa-cart-shopping"></i></button>
            };
        }
        let rows = ctx
            .props()
            .list
            .iter()
            .enumerate()
            .map(|(index, id)| self.make_row(index, *id, ctx))
            .collect::<Html>();

        let close = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::Close
        });
        let download = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::Download
        });

        html! {<div id="cart">
        <button onclick={close} id="close-cart"><i class="fa-solid fa-x"></i></button>
        {rows}
        <button onclick={download} id="download-cart" title="Descargar hoja"><i class="fa-solid fa-file-zipper"></i> </button>
        </div>}
    }
}

impl Comp {
    fn make_row(&self, index: usize, id: usize, ctx: &Context<Self>) -> Html {
        let click_up = button_callback!(ctx, Msg::MoveUp, index);
        let click_down = button_callback!(ctx, Msg::MoveDown, index);
        let delete = button_callback!(ctx, Msg::RemoveIndex, index);
        let toggle = button_callback!(ctx, Msg::ToggleShow, id);

        let with_preview = self.showing.contains(&id);

        let eye = if with_preview {
            html! {<i class="fa-solid fa-eye-slash"></i>}
        } else {
            html! {<i class="fa-solid fa-eye"></i>}
        };

        let (problem_contents, title) = ctx
            .props()
            .db
            .iter()
            .find_map(|problem| {
                if problem.id == id {
                    Some((problem.enunciado.clone(), problem.titulo.clone()))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                log::error!("El problema {id} no está en la base de datos!");
                (String::new(), String::new())
            });

        html! {
            <div class="cart-row">
            <button onclick={click_up}><i class="fa-solid fa-up-long"></i></button>
            <button onclick={click_down}><i class="fa-solid fa-down-long"></i></button>
            <button onclick={delete}><i class="fa-solid fa-trash-can"></i></button>
            <button onclick={toggle} title="Mostrar el enunciado">{eye}</button>
            <h3>{AttrValue::from(title)}</h3>
            if with_preview {
                <crate::raw_html::Comp inner_html={problem_contents} tag="div" />
            }

            </div>
        }
    }
}
