use web_sys::Element;
use yew::{
    prelude::*,
    virtual_dom::{VNode, VTag},
};

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub inner_html: String,
    pub tag: &'static str,
}

pub struct Comp {
    node_ref: NodeRef,
}

impl Component for Comp {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut tag = VTag::new(ctx.props().tag);
        tag.node_ref = self.node_ref.clone();
        VNode::from(tag)
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&ctx.props().inner_html);
    }

    // fn view(&self) -> Html {
    //     // create the parent element and store a reference to it
    //     html! {
    //         <div ref=self.node_ref.clone()/>
    //     }
    // }

    // fn rendered(&mut self, _first_render: bool) {
    //     let el = self.node_ref.cast::<Element>().unwrap();
    //     el.set_inner_html(&self.props.inner_html);
    // }
}
