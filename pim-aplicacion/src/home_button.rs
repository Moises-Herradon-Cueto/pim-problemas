use std::marker::PhantomData;

use yew::{prelude::*, virtual_dom::VChild};

pub struct With<T> {
    _marker: PhantomData<T>,
}

pub enum UpdateMsg {}

#[derive(Properties, PartialEq, Clone)]
pub struct Props<TProps: PartialEq> {
    pub return_cb: Callback<()>,
    pub props: TProps,
}

impl<T: Component> Component for With<T>
where
    T::Properties: Clone,
{
    type Message = UpdateMsg;
    type Properties = Props<T::Properties>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _marker: PhantomData::<T>,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let return_cb = ctx.props().return_cb.clone();
        let return_button = ctx.link().batch_callback(move |_: MouseEvent| {
            return_cb.emit(());
            None
        });
        let t: VChild<T> = VChild::new(ctx.props().props.clone(), NodeRef::default(), None);
        html! {
            <>
            <button onclick={return_button}>{"Volver al inicio"}</button>
            {t}
            </>
        }
    }
}
