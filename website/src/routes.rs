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

    // FIXME: workaround for deploy
    #[at("/online-fruit-marketplace")]
    SubRoute,
}

// FIXME: workaround for deploy
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum SubRoute {
    #[at("/online-fruit-marketplace/")]
    Home,
    #[at("/online-fruit-marketplace/product/:id")]
    ProductDetail { id: i32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}
