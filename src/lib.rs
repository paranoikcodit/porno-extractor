pub use extractor_core::{Extractor, Model, PageType, SearchResult, Video, VideoPreview};
pub use pornhub_extractor;
pub use xvideos_extractor;

#[enum_dispatch::enum_dispatch(Extractor)]
#[derive(Clone)]
pub enum Extractors {
    XVideos(xvideos_extractor::XvideoExtractor),
    Pornhub(pornhub_extractor::PornhubExtractor),
}
