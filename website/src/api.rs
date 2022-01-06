use anyhow::Error;
use reqwasm::http::Request;

use crate::types::Product;

pub async fn get_products() -> Result<Vec<Product>, Error> {
    let response = Request::get("./products/products.json").send().await?;
    let products: Vec<Product> = response.json().await?;
    log::debug!("{:?}", products);
    Ok(products)
}

pub async fn get_product(id: i32) -> Result<Product, Error> {
    let response = Request::get(&format!("/products/{}.json", id))
        .send()
        .await?;
    let product: Product = response.json().await?;
    log::debug!("{:?}", product);
    Ok(product)
}
