use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod channel;
pub mod search;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XvConfData {
    pub action: String,
    #[serde(rename = "other_locales")]
    pub other_locales: Value,
    #[serde(rename = "show_disclaimer")]
    pub show_disclaimer: bool,
    pub sponsors: Value,
    #[serde(rename = "id_video")]
    pub id_video: i64,
    #[serde(rename = "uploader_id")]
    pub uploader_id: i64,
    pub uploader: String,
    #[serde(rename = "uploader_url")]
    pub uploader_url: String,
    #[serde(rename = "video_tags")]
    pub video_tags: Vec<String>,
    #[serde(rename = "video_models")]
    pub video_models: Value,
    pub main_cat: Option<String>,
    pub main_cats: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoData {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    pub content_url: String,
    pub description: String,
    pub duration: String,
    pub interaction_statistic: InteractionStatistic,
    pub name: String,
    pub thumbnail_url: Vec<String>,
    pub upload_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionStatistic {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub interaction_type: InteractionType,
    pub user_interaction_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionType {
    #[serde(rename = "@type")]
    pub type_field: String,
}
