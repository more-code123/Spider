mod structs;

use crate::structs::{ApiResponse, Article, Author};
use reqwest::{Error, Response};
use scraper::Html;
use serde_json::Number;
use std::ops::Add;

pub async fn send_request(url: String) -> Result<Html, Error> {
    let response = reqwest::get(url).await?;
    let html = Html::parse_document(&response.text().await?);
    Ok(html)
}

pub async fn parse_rank_list(response: Response) -> Result<Vec<Article>, Error> {
    let api_response = response.json::<ApiResponse>().await?;
    let data = api_response.data;

    Ok(data)
}