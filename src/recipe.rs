// Based on Leptos crate fetch example

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub cuisine: String,
    pub ingredients: Vec<String>,
    pub cooking_time_minutes: i64,
    pub prep_time_minutes: i64,
    pub servings: i64,
    pub calories_per_serving: i64,
    pub dietary_restrictions: Vec<String>,
}

pub async fn fetch(endpoint: String) -> Result<Recipe, Error> {
    use reqwasm::http::Request;

    let endpoint = format!("http://localhost:3000/api/v1/recipe/{}", endpoint);
    let result = Request::get(&endpoint).send().await?.json().await?;
    Ok(result)
}
