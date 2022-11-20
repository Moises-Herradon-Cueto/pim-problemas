use material_yew::select::{ListIndex, MatSelect, SelectedDetail};

use material_yew::MatListItem;
use parse_lib::Fields;
use yew::prelude::*;

pub struct Comp;

pub enum Msg {
    FieldType(Fields),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub select_cb: Callback<Fields>,
}

impl Component for Comp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Msg::FieldType(field) = msg;
        ctx.props().select_cb.emit(field);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onselected = ctx.link().batch_callback(|x: SelectedDetail| {
            let selection = x.index;
            if let ListIndex::Single(Some(i)) = selection {
                Fields::try_from(i).map_or_else(
                    |_| {
                        log::info!("Field {i} out of range");
                        vec![]
                    },
                    |field| vec![Msg::FieldType(field)],
                )
            } else {
                log::info!("{selection:?}");
                vec![]
            }
        });

        let fields_list: Html = Fields::ALL
            .into_iter()
            .filter(|x| x.is_in_template())
            .map(|field| {
                html! {
                    <MatListItem>{field}</MatListItem>
                }
            })
            .collect();

        html! {
                      <MatSelect {onselected}>
                        {fields_list}
                    </MatSelect>
        }
    }
}
