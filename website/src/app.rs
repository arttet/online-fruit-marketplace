use yew::prelude::{html, Component, Context, Html};
use yew_router::prelude::{BrowserRouter, Redirect, Switch};

use crate::components::Navbar;
use crate::pages::{Home, ProductDetail};
use crate::routes::{Route, SubRoute};
use crate::types::{CartProduct, Product};

struct State {
    cart_products: Vec<CartProduct>,
}

pub struct App {
    state: State,
}

pub enum Msg {
    AddToCart(Product),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let cart_products = vec![];

        Self {
            state: State { cart_products },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddToCart(product) => {
                let cart_product = self
                    .state
                    .cart_products
                    .iter_mut()
                    .find(|cp: &&mut CartProduct| cp.product.id == product.id);

                if let Some(cp) = cart_product {
                    cp.quantity += 1;
                } else {
                    self.state.cart_products.push(CartProduct {
                        product: product.clone(),
                        quantity: 1,
                    })
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cart_products = self.state.cart_products.clone();
        let handle_add_to_cart = ctx
            .link()
            .callback(|product: Product| Msg::AddToCart(product));

        let switch = move |switch: &Route| -> Html {
            match switch {
                Route::ProductDetail { id } => {
                    html! {<ProductDetail id={ *id } on_add_to_cart={ handle_add_to_cart.clone() } />}
                }
                Route::Home => {
                    html! {<Home cart_products={ cart_products.clone() } on_add_to_cart={ handle_add_to_cart.clone() } />}
                }
                Route::NotFound => {
                    html! { <h1>{ "404" }</h1> }
                }
                // FIXME: workaround for deploy
                Route::SubRoute => {
                    let cart_products = cart_products.clone();
                    let handle_add_to_cart = handle_add_to_cart.clone();

                    let sub_switch = move |switch: &SubRoute| -> Html {
                        match switch {
                            SubRoute::ProductDetail { id } => {
                                html! {<ProductDetail id={ *id } on_add_to_cart={ handle_add_to_cart.clone() } />}
                            }
                            SubRoute::Home => {
                                html! {<Home cart_products={ cart_products.clone() } on_add_to_cart={ handle_add_to_cart.clone() } />}
                            }
                            SubRoute::NotFound => {
                                html! { <Redirect<Route> to={ Route::NotFound }/> }
                            }
                        }
                    };

                    html! {
                        <Switch<SubRoute> render={ Switch::render(sub_switch) } />
                    }
                }
            }
        };

        html! {
            <BrowserRouter>
                <main>
                    <Navbar cart_products={ self.state.cart_products.clone() } />
                    <Switch<Route> render={ Switch::render(switch) } />
                </main>
            </BrowserRouter>
        }
    }
}
