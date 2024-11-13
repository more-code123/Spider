use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct BpiResponse {
    pub code: i32,
    pub message: String,
    pub ttl: i32,
    pub data: Data,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Data {
    pub note: String,
    pub list: Vec<VideoInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct VideoInfo {
    pub aid: i64,
    pub videos: i32,
    pub tid: i32,
    pub tname: String,
    pub copyright: i32,
    pub pic: String,
    pub title: String,
    pub pubdate: i64,
    pub ctime: i64,
    pub desc: String,
    pub state: i32,
    pub duration: i32,
    #[serde(default = "default_i64")]
    pub mission_id: i64,
    pub rights: Rights,
    pub owner: Owner,
    pub stat: Stat,
    pub dynamic: String,
    pub cid: i64,
    pub dimension: Dimension,
    #[serde(default = "default_i64")]
    pub season_id: i64,
    pub short_link_v2: String,
    pub first_frame: String,
    #[serde(default = "default_string")]
    pub pub_location: String,
    pub cover43: String,
    pub bvid: String,
    pub score: i32,
    pub enable_vt: i32,
}

fn default_string() -> String {
    String::new()
}

fn default_i64() -> i64 {
    0
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Rights {
    pub bp: i32,
    pub elec: i32,
    pub download: i32,
    pub movie: i32,
    pub pay: i32,
    pub hd5: i32,
    pub no_reprint: i32,
    pub autoplay: i32,
    pub ugc_pay: i32,
    pub is_cooperation: i32,
    pub ugc_pay_preview: i32,
    pub no_background: i32,
    pub arc_pay: i32,
    pub pay_free_watch: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Owner {
    pub mid: i64,
    pub name: String,
    pub face: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Stat {
    pub aid: i64,
    pub view: i64,
    pub danmaku: i64,
    pub reply: i64,
    pub favorite: i64,
    pub coin: i64,
    pub share: i64,
    pub now_rank: i32,
    pub his_rank: i32,
    pub like: i64,
    pub dislike: i32,
    pub vt: i32,
    pub vv: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Dimension {
    pub width: i32,
    pub height: i32,
    pub rotate: i32,
}
