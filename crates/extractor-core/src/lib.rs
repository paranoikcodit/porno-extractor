#![feature(associated_type_defaults)]

use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub use anyhow;

#[macro_export]
macro_rules! select {
    ($selector: expr) => {
        ::scraper::Selector::parse($selector).unwrap()
    };
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub avatar_url: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoPreview {
    pub id: String,
    pub title: String,
    pub thumbnail_url: String,
    pub duration: String,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub thumbnail_url: String,
    pub video_url: String,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub duration: Duration,
    pub description: String,
    pub model: String,
    pub title: String,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            id: String::new(),
            duration: Duration::zero(),
            thumbnail_url: String::new(),
            video_url: String::new(),
            description: String::new(),
            model: String::new(),
            title: String::new(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SearchResult {
    pub channels: Vec<String>,
    // pub models: Vec<String>,
    pub keywords: Vec<String>,
}

pub enum PageType {
    Model(String),
    Tag(String),
    Search(String),
    Main,
}

type MyResult<T> = anyhow::Result<T>;

#[enum_dispatch::enum_dispatch]
pub trait Extractor {
    async fn get_search_variants(&self, query: &str) -> MyResult<SearchResult>;
    async fn get_recommendations(&self, video_id: &str) -> MyResult<Vec<VideoPreview>>;
    async fn get_video(&self, video_id: &str) -> MyResult<Video>;
    // async fn get_videos_model(&self, model: &str) -> anyhow::Result<Vec<Video>>;
    async fn get_videos_page(&self, page: u16, page_type: PageType) -> MyResult<Vec<VideoPreview>>;
    async fn get_model(&self, model: &str) -> MyResult<Model>;
}
