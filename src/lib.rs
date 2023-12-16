pub use extractor_core::{Extractor, Model, PageType, SearchResult, Video, VideoPreview};
pub use pornhub_extractor;
pub use xvideos_extractor;

#[enum_dispatch::enum_dispatch(Extractor)]
pub enum Extractors {
    XVideos(xvideos_extractor::XvideoExtractor),
    Pornhub(pornhub_extractor::PornhubExtractor),
}

impl Clone for Extractors {
    fn clone(&self) -> Self {
        match self {
            Extractors::Pornhub(x) => Extractors::Pornhub(x.clone()),
            Extractors::XVideos(x) => Extractors::XVideos(x.clone()),
        }
    }
}
