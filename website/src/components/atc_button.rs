use yew::prelude::{html, Callback, Component, Context, Html, Properties};

use crate::types::Product;

pub struct AtcButton {}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

pub enum Msg {
    AddToCart,
}

impl Component for AtcButton {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddToCart => ctx.props().on_add_to_cart.emit(ctx.props().product.clone()),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::AddToCart);

        html! {
            <button class="product_atc_button" onclick={ onclick }>{"Add To Cart"}</button>
        }
    }
}
