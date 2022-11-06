use yew::prelude::*;

pub struct Update;

pub enum UpdateMsg {}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub return_cb: Callback<()>,
}

impl Component for Update {
    type Message = UpdateMsg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let return_cb = ctx.props().return_cb.clone();
        let return_button = ctx.link().batch_callback(move |_: MouseEvent| {
            return_cb.emit(());
            None
        });
        html! {
            <>
            <button onclick={return_button}>{"Volver al inicio"}</button>
            </>
        }
    }
}
