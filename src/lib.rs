pub use extractor_core::{Extractor, Model, PageType, SearchResult, Video, VideoPreview};
pub use pornhub_extractor;
use pornhub_extractor::PornhubExtractor;
pub use xvideos_extractor;
use xvideos_extractor::XvideoExtractor;

#[enum_dispatch::enum_dispatch(Extractor)]
pub enum Extractors {
    XVideos(XvideoExtractor),
    Pornhub(PornhubExtractor),
}

impl Clone for Extractors {
    fn clone(&self) -> Self {
        match self {
            Extractors::Pornhub(x) => Extractors::Pornhub(x.clone()),
            Extractors::XVideos(x) => Extractors::XVideos(x.clone()),
        }
    }
}
