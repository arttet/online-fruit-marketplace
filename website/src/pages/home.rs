use anyhow::Error;
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::api::get_products;
use crate::components::ProductCard;
use crate::types::{CartProduct, Product};

struct State {
    products: Vec<Product>,
    get_products_error: Option<Error>,
    get_products_loaded: bool,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub cart_products: Vec<CartProduct>,
    pub on_add_to_cart: Callback<Product>,
}

pub struct Home {
    props: Props,
    state: State,
}

pub enum Msg {
    GetProducts,
    GetProductsSuccess(Vec<Product>),
    GetProductsError(Error),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let products: Vec<Product> = vec![];

        ctx.link().send_message(Msg::GetProducts);

        Self {
            props: ctx.props().clone(),
            state: State {
                products,
                get_products_error: None,
                get_products_loaded: false,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetProducts => {
                self.state.get_products_loaded = false;
                ctx.link().send_future(async {
                    match get_products().await {
                        Ok(products) => Msg::GetProductsSuccess(products),
                        Err(err) => Msg::GetProductsError(err),
                    }
                });
                true
            }
            Msg::GetProductsSuccess(products) => {
                self.state.products = products;
                self.state.get_products_loaded = true;
                true
            }
            Msg::GetProductsError(error) => {
                self.state.get_products_error = Some(error);
                self.state.get_products_loaded = true;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {
                    <ProductCard product={product.clone()} on_add_to_cart={self.props.on_add_to_cart.clone()} />
                }
            })
            .collect();

        if !self.state.get_products_loaded {
            html! {
              <div>{"Loading ..."}</div>
            }
        } else if let Some(_) = self.state.get_products_error {
            html! {
              <div>
                <span>{"Error loading products! :("}</span>
              </div>
            }
        } else {
            html! {
                <div class="product_card_list">{products}</div>
            }
        }
    }
}
