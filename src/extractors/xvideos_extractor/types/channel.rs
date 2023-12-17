use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideos {
    #[serde(rename = "main_cats")]
    pub main_cats: Value,
    #[serde(rename = "main_cat")]
    pub main_cat: Option<String>,
    #[serde(rename = "main_cats_url")]
    pub main_cats_url: Option<MainCatsUrl>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "show_up_only")]
    pub show_up_only: i64,
    #[serde(rename = "nb_videos")]
    pub nb_videos: i64,
    #[serde(rename = "nb_per_page")]
    pub nb_per_page: i64,
    #[serde(rename = "current_page")]
    pub current_page: i64,
    pub videos: Vec<Video>,
    #[serde(rename = "is_model")]
    pub is_model: bool,
    #[serde(rename = "is_channel")]
    pub is_channel: bool,
    pub result: bool,
    pub code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainCatsUrl {
    pub straight: String,
    pub shemale: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: i64,
    pub u: String,
    pub i: String,
    pub il: String,
    #[serde(rename = "if")]
    pub if_field: String,
    pub ip: String,
    pub c: i64,
    pub tf: String,
    pub t: String,
    pub d: String,
    pub r: String,
    pub n: String,
    pub v: i64,
    pub vim: i64,
    pub hm: i64,
    pub h: i64,
    pub hp: i64,
    pub td: i64,
    pub fk: i64,
    pub ve: i64,
    pub ui: i64,
    pub p: String,
    pub pn: String,
    pub pu: String,
    pub ch: bool,
    pub pm: bool,
    pub ut: Value,
    pub iu: bool,
}
