use juejin::parse_rank_list;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let base_url = "https://api.juejin.cn/content_api/v1/content/article_rank?category_id=1&type=hot";
    let response = reqwest::get(base_url).await?;
    let rank_list = parse_rank_list(response).await?;
    for article in rank_list {
        println!("{:#?}", article.content_counter);
    }
    Ok(())
}