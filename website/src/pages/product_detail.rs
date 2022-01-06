use anyhow::Error;
use yew::prelude::{html, Component, Context, Html, Properties, Callback};

use crate::api::get_product;
use crate::components::AtcButton;
use crate::types::Product;

struct State {
    product: Option<Product>,
    get_product_error: Option<Error>,
    get_product_loaded: bool,
}

pub struct ProductDetail {
    props: Props,
    state: State,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: i32,
    pub on_add_to_cart: Callback<Product>,
}

pub enum Msg {
    GetProduct,
    GetProductSuccess(Product),
    GetProductError(Error),
}

impl Component for ProductDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetProduct);

        Self {
            props: ctx.props().clone(),
            state: State {
                product: None,
                get_product_error: None,
                get_product_loaded: false,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetProduct => {
                let id = self.props.id;
                self.state.get_product_loaded = false;
                ctx.link().send_future(async move {
                    match get_product(id).await {
                        Ok(product) => Msg::GetProductSuccess(product),
                        Err(err) => Msg::GetProductError(err),
                    }
                });
                true
            }
            Msg::GetProductSuccess(product) => {
                self.state.product = Some(product);
                self.state.get_product_loaded = true;
                true
            }
            Msg::GetProductError(error) => {
                self.state.get_product_error = Some(error);
                self.state.get_product_loaded = true;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(ref product) = self.state.product {
            html! {
                <div class="product_detail_container">
                    <img class="product_detail_image" src={product.image.clone()}/>
                    <div class="product_card_name">{&product.name}</div>
                    <div style="margin: 10px 0; line-height: 24px;">{&product.description}</div>
                    <div class="product_card_price">{"$"}{&product.price}</div>
                    <AtcButton product={product.clone()} on_add_to_cart={self.props.on_add_to_cart.clone()} />
                </div>
            }
        } else if !self.state.get_product_loaded {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else {
            html! {
                <div>
                    <span>{"Error loading product! :("}</span>
                </div>
            }
        }
    }
}
