use anyhow::anyhow;
use chrono::Duration;
// use extractor_core::{select, Extractor, Model, PageType, SearchResult, Video, VideoPreview};
use scraper::{Html, Selector};
use serde_json::Value;
use types::{channel::ChannelVideos, search::SearchSuggest, XvConfData};

use crate::{select, Extractor, Model, PageType, SearchResult, Video, VideoPreview};

use self::types::VideoData;

// use cratetypes::VideoData;

pub mod types;

#[derive(Clone, Debug, Default)]
pub struct XvideoExtractor {
    client: reqwest::Client,
}

fn parse_duration(duration: &str) -> Duration {
    let duration = (&duration[2..])
        .split(|c| c == 'H' || c == 'M' || c == 'S')
        .collect::<Vec<&str>>();

    let hours = duration
        .get(0)
        .and_then(|h| h.parse::<i64>().ok())
        .unwrap_or(0);
    let minutes = duration
        .get(1)
        .and_then(|m| m.parse::<i64>().ok())
        .unwrap_or(0);
    let seconds = duration
        .get(2)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);

    Duration::hours(hours) + Duration::minutes(minutes) + Duration::seconds(seconds)
}

impl XvideoExtractor {
    fn get_videos(&self, body: &str) -> anyhow::Result<Vec<VideoPreview>> {
        let selector = Selector::parse(".mozaique > .thumb-block").unwrap();
        let html = Html::parse_document(body);

        let videos = html
            .select(&selector)
            .filter_map(|element| {
                if let Some(link) = element.select(&select!("* > p.title > a")).next() {
                    let title = link.attr("title").unwrap().to_string();
                    let video_id = link
                        .attr("href")
                        .unwrap()
                        .replace("/", "")
                        .replace("_", "")
                        .replace("-", "");

                    let duration = element
                        .select(&select!("* > .duration"))
                        .next()
                        .unwrap()
                        .text()
                        .collect::<String>();

                    let thumb_url = element
                        .select(&select!(".thumb > a"))
                        .next()
                        .unwrap()
                        .attr("href")
                        .unwrap();

                    Some(VideoPreview {
                        title,
                        duration,
                        id: video_id,
                        thumbnail_url: thumb_url.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<VideoPreview>>();

        Ok(videos)
    }

    async fn get_video_info(&self, video_id: &str) -> anyhow::Result<VideoData> {
        let response = self
            .client
            .get(format!("https://www.xvideos.com/{video_id}/_"))
            .send()
            .await?
            .text()
            .await?;

        let video_data = response
            .split("<script type=\"application/ld+json\">")
            .last()
            .unwrap()
            .split("</script>")
            .next()
            .unwrap()
            .trim()
            .replace(';', "");

        serde_json::from_str::<VideoData>(&video_data).map_err(|e| e.into())
    }

    async fn get_xv_conf(&self, url: &str) -> anyhow::Result<Value> {
        let response = self.client.get(url).send().await?.text().await?;

        let match_ = regex::Regex::new(r"xv.conf=\{.*\};")?;
        let matched = match_
            .find(&response)
            .ok_or(anyhow!("Failed to find xv_conf from page"))?
            .as_str()
            .replace(";", "")
            .replace("xv.conf=", "");

        serde_json::from_str::<Value>(&matched).map_err(|e| e.into())
    }

    async fn get_xv_conf_data(&self, url: &str) -> anyhow::Result<XvConfData> {
        let xv_conf = self.get_xv_conf(url).await?;

        serde_json::from_value(xv_conf["data"].clone()).map_err(|e| e.into())
    }
}

// #[async_trait(?Send)]
impl Extractor for XvideoExtractor {
    async fn get_recommendations(&self, video_id: &str) -> anyhow::Result<Vec<VideoPreview>> {
        let body = self
            .client
            .get(format!("https://xvideos.com/{video_id}/_"))
            .send()
            .await?
            .text()
            .await?;

        let video_related_regex = regex::Regex::new(r"video_related=\[\{(.*)\}\];")?;
        let video_related = video_related_regex
            .find(&body)
            .ok_or(anyhow!("Failed to resolve related videos"))?
            .as_str()
            .replace("video_related=", "")
            .replace(";", "");

        Ok(serde_json::from_str::<Value>(&video_related)
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|element| VideoPreview {
                duration: element["d"].as_str().unwrap().to_string(),
                id: format!("video{}", element["id"].as_i64().unwrap()),
                thumbnail_url: element["i"].as_str().unwrap().to_string(),
                title: element["t"].as_str().unwrap().to_string(),
            })
            .collect())
    }

    async fn get_model(&self, model: &str) -> anyhow::Result<Model> {
        let response = self
            .client
            .get(format!("https://www.xvideos.com/channels/{model}"))
            .send()
            .await?
            .text()
            .await?;

        let html = Html::parse_document(&response);

        let mut model = Model::default();

        if let Some(profile_title_element) = html.select(&select!("#profile-title")).next() {
            model.avatar_url = profile_title_element
                .select(&select!(".profile-pic > img"))
                .next()
                .and_then(|img| img.attr("src").map(|s| s.to_string()))
                .unwrap();
            model.name = profile_title_element
                .select(&select!("h2 > strong"))
                .next()
                .map(|element| element.text().collect::<String>().trim().to_string())
                .unwrap();
        } else {
            return Err(anyhow!("Failed to find profile_title element"));
        }

        if let Some(header_about_me) = html.select(&select!("#header-about-me")).next() {
            model.description = header_about_me.text().collect::<String>();
        } else {
            return Err(anyhow!("Failed to find description element"));
        }

        Ok(model)
    }

    async fn get_search_variants(&self, query: &str) -> anyhow::Result<SearchResult> {
        let search_suggest = self
            .client
            .get(format!("https://www.xvideos.com/search-suggest/{query}"))
            .send()
            .await?
            .json::<SearchSuggest>()
            .await?;

        Ok(SearchResult {
            channels: search_suggest
                .channel
                .iter()
                .map(|channel| channel.f.clone())
                .collect(),
            keywords: search_suggest
                .keywords
                .iter()
                .map(|keyword| keyword.n.clone())
                .collect(),
        })
    }

    async fn get_video(&self, video_id: &str) -> anyhow::Result<Video> {
        let video_url = format!("https://xvideos.com/{video_id}/_");

        let video_info = self.get_video_info(video_id).await?;
        let xv_conf_data = self.get_xv_conf_data(&video_url).await?;

        Ok(Video {
            description: video_info.description,
            model: xv_conf_data.uploader,
            thumbnail_url: video_info.thumbnail_url[0].clone(),
            video_url: video_info.content_url,
            duration: parse_duration(&video_info.duration),
            title: video_info.name,
            id: video_id.to_string(),
        })
    }

    async fn get_videos_page(
        &self,
        page: u16,
        page_type: PageType,
    ) -> anyhow::Result<Vec<VideoPreview>> {
        let page = page - 1;
        let url = match page_type {
            PageType::Model(model) => {
                let xv_conf = self
                    .get_xv_conf(&format!("https://xvideos.com/channels/{model}"))
                    .await?;
                let pagefilter = xv_conf["dyn"]["pagefilter"]
                    .clone()
                    .as_str()
                    .unwrap()
                    .to_string();

                let videos = self
                    .client
                    .post(format!(
                        "https://www.xvideos.com/channels/{model}/videos/best/{pagefilter}/{page}"
                    ))
                    .body(format!("main_cats[]={pagefilter}"))
                    .send()
                    .await?
                    .json::<ChannelVideos>()
                    .await?;

                return Ok(videos
                    .videos
                    .iter()
                    .map(|video| VideoPreview {
                        duration: video.d.clone(),
                        id: format!("video{}", video.id),
                        thumbnail_url: video.ip.clone(),
                        title: video.tf.clone(),
                    })
                    .collect::<Vec<VideoPreview>>());
            }
            PageType::Tag(tag) => {
                let mut url = format!("https://xvideos.com/tags/{tag}");

                if page > 0 {
                    url = format!("{url}/{page}");
                }

                url
            }
            PageType::Search(query) => {
                let mut url = format!("https://xvideos.com/?k={query}");

                if page > 0 {
                    url = format!("{url}&p={page}");
                }

                url
            }
            PageType::Main => {
                let mut url = "https://xvideos.com".to_string();

                if page > 0 {
                    url = format!("{url}/new/{page}");
                }

                url
            }
        };

        self.get_videos(&self.client.get(url).send().await?.text().await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Extractor, PageType, XvideoExtractor};
    // use extractor_core::{Extractor, PageType};

    #[tokio::test]
    async fn get_recommendations_test() {
        let response = XvideoExtractor::default()
            .get_recommendations("video36230173")
            .await;

        // assert!(response.is_ok());
        assert!(response.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn get_model() {
        let response = XvideoExtractor::default()
            .get_model("testedefudelidade")
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap().name, "Teste De Fudelidade");
    }

    #[tokio::test]
    async fn get_videos_page_model() {
        let model_page_type = PageType::Model("testedefudelidade".to_string());
        let response = XvideoExtractor::default()
            .get_videos_page(35, model_page_type)
            .await;

        // assert!(response.is_ok());
        assert!(response.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn get_videos_page_main() {
        let main_page_type = PageType::Main;
        let response = XvideoExtractor::default()
            .get_videos_page(1, main_page_type)
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn get_videos_page_search() {
        let search_page_type = PageType::Search("deepthroat".to_string());
        let response = XvideoExtractor::default()
            .get_videos_page(1, search_page_type)
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn get_videos_page_tag() {
        let tag_page_type = PageType::Tag("deepthroat".to_string());
        let response = XvideoExtractor::default()
            .get_videos_page(1, tag_page_type)
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn get_video_test() {
        assert!(
            XvideoExtractor::default()
                .get_video("video36230173")
                .await
                .unwrap()
                .model
                == "cumbiz"
        )
    }
}
