use yew::prelude::*;

pub struct Comp;

pub struct Click;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub cb: Callback<()>,
}

impl Component for Comp {
    type Message = Click;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().cb.emit(());
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Click
        });
        html! {
            <button {onclick} class="icon-button" title="Recargar la base de datos">
                <i class="fa-solid fa-arrows-rotate"></i>
            </button>
        }
    }
}
