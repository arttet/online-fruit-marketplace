use yew::prelude::{html, classes, Callback, Component, Context, Html, Properties};

use yew_router::components::Link;

use crate::components::AtcButton;
use crate::routes::Route;
use crate::types::Product;

pub struct ProductCard {
    props: Props,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

impl Component for ProductCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="product_card_container">
                <Link<Route> classes={classes!("subtitle", "is-block")} to={Route::ProductDetail { id: self.props.product.id }}>
                    <img class="product_card_image" src={self.props.product.image.clone()}/>
                    <div class="product_card_name">{&self.props.product.name}</div>
                    <div class="product_card_price">{"$"}{&self.props.product.price}</div>
                </Link<Route>>
                <AtcButton product={self.props.product.clone()} on_add_to_cart={self.props.on_add_to_cart.clone()} />
            </div>
        }
    }
}
