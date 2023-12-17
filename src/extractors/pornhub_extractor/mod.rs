use crate::Extractor;

#[derive(Default, Clone)]
pub struct PornhubExtractor {
    pub client: reqwest::Client,
}

impl PornhubExtractor {}

impl Extractor for PornhubExtractor {
    async fn get_search_variants(&self, query: &str) -> anyhow::Result<crate::SearchResult> {
        todo!()
    }

    async fn get_recommendations(
        &self,
        video_id: &str,
    ) -> anyhow::Result<Vec<crate::VideoPreview>> {
        todo!()
    }

    async fn get_video(&self, video_id: &str) -> anyhow::Result<crate::Video> {
        todo!()
    }

    async fn get_videos_page(
        &self,
        page: u16,
        page_type: crate::PageType,
    ) -> anyhow::Result<Vec<crate::VideoPreview>> {
        todo!()
    }

    async fn get_model(&self, model: &str) -> anyhow::Result<crate::Model> {
        todo!()
    }
}
