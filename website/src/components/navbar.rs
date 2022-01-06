use yew::prelude::{html, Component, Context, Html, Properties};

use crate::types::CartProduct;

pub struct Navbar {}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
}

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cart_value = ctx
            .props()
            .cart_products
            .iter()
            .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));

        html! {
            <div class="navbar">
                <div class="navbar_title">{"Online Fruit Marketplace"}</div>
                <div class="navbar_cart_value">{format!("${:.2}", cart_value)}</div>
            </div>
        }
    }
}
