use material_yew::checkbox::MatCheckbox;
use pim_lib::Fields;
use yew::prelude::*;

pub struct Comp;

pub enum Msg {}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub show_cb: Callback<(bool, Fields)>,
    pub show: [bool; Fields::N],
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form id="column-select">
            {Self::buttons(ctx)}
            </form>
        }
    }
}

impl Comp {
    fn buttons(ctx: &Context<Self>) -> Html {
        Fields::ALL
            .into_iter()
            .filter(|x| x.is_in_template())
            .map(|field| {
                let label = format!("{field}-toggle");
                let show_cb = ctx.props().show_cb.clone();
                let onchange = Callback::from(move |value: bool| {
                    show_cb.emit((value, field));
                });

                html! {
                    <div class="show-container">
                    <MatCheckbox {onchange} checked={ctx.props().show[field as usize]}/>
                    <div>
                    <label for={label}>
                    {format!("Mostrar {field}")}</label>
                    </div>
                    </div>
                }
            })
            .collect()
    }
}
