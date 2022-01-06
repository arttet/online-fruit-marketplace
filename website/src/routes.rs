//! Routes by yew_router

use yew_router::prelude::Routable;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/product/:id")]
    ProductDetail { id: i32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}
