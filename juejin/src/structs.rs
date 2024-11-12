use std::ops::Add;
use serde_json::Number;
use serde::Deserialize;
use crate::send_request;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ApiResponse {
    pub data: Vec<Article>
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Article {
    pub author_counter: AuthorCounter,
    pub content: Content,
    pub content_counter: ContentCounter,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AuthorCounter {
    followee: Number,
    follower: Number,
    hot_rank: Number,
    level: Number,
    like: Number,
    power: Number,
    publish: Number,
    view: Number,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Content {
    pub author_id: String,
    pub brief: String,
    pub category_id: String,
    pub content_id: String,
    pub ctime: Number,
    pub format: String,
    pub item_type: Number,
    pub mtime: Number,
    pub status: Number,
    pub tag_ids: Vec<String>,
    pub title: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ContentCounter {
    collect: Number,
    comment_count: Number,
    hot_rank: Number,
    interact_count: Number,
    like: Number,
    view: Number,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Author {
    // 用户名
    pub(crate) name: String,
    // 用户id
    pub(crate) user_id: String,
    // 等级
    pub(crate) level: Number,
    // 掘力值
    pub(crate) power: Number,
    // 粉丝数
    pub(crate) followee: Number,
    // 关注人数
    pub(crate) follower: Number,
    // 文章被点赞数
    pub(crate) like: Number,
}

impl Author {
    pub fn init(user_id: String) -> Author {
        Author {
            name: String::from(""),
            user_id,
            level: Number::from(0),
            power: Number::from(0),
            followee: Number::from(0),
            follower: Number::from(0),
            like: Number::from(0),
        }
    }
    pub async fn set_author(mut self) {
        let user_url = String::from("https://juejin.cn/user/");
        let user_id = self.user_id;
        let html = send_request(user_url.add(user_id.as_str())).await.unwrap();
        todo!("解析并设置作者信息");
    }
}