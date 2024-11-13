mod structs;

use reqwest::Error;
use crate::structs::BpiResponse;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let base_url = "https://api.bilibili.com/x/web-interface/ranking/v2";
    let response = reqwest::get(base_url).await?.json::<BpiResponse>().await?;
    let info_list = response.data.list;
    for (index, item) in info_list.iter().enumerate() {
        println!("{:^5}{:^10}{:^20}\x1B]8;;{}\x1B\\{:^}\x1B]8;;\x1B\\", index + 1, item.pub_location, item.owner.name, item.short_link_v2, item.title)
    }

    Ok(())
}
