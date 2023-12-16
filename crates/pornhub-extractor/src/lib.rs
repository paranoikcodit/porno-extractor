use extractor_core::Extractor;

#[derive(Default)]
pub struct Pornhub {
    pub client: reqwest::Client,
}

impl Pornhub {}

impl Extractor for Pornhub {
    async fn get_search_variants(
        &self,
        query: &str,
    ) -> anyhow::Result<extractor_core::SearchResult> {
        todo!()
    }

    async fn get_recommendations(
        &self,
        video_id: &str,
    ) -> anyhow::Result<Vec<extractor_core::VideoPreview>> {
        todo!()
    }

    async fn get_video(&self, video_id: &str) -> anyhow::Result<extractor_core::Video> {
        todo!()
    }

    async fn get_videos_page(
        &self,
        page: u16,
        page_type: extractor_core::PageType,
    ) -> anyhow::Result<Vec<extractor_core::VideoPreview>> {
        todo!()
    }

    async fn get_model(&self, model: &str) -> anyhow::Result<extractor_core::Model> {
        todo!()
    }
}
