use aidoku::{
    prelude::*,
    std::{net::Request, html::Node, html::Html, String, Vec, error::AidokuError},
    manga::{Manga, MangaPageResult, Chapter, Page},
};

#[register_source]
pub fn register() -> MangaSource {
    MangaSource::new(MangakakalotSource)
}

pub struct MangakakalotSource;

impl aidoku::manga::Source for MangakakalotSource {
    fn get_manga_list(&self, _filters: Vec<aidoku::filter::Filter>, _page: i32) -> Result<MangaPageResult, AidokuError> {
        Ok(MangaPageResult {
            manga: Vec::new(),
            has_more: false,
        })
    }

    fn get_manga_details(&self, _id: String) -> Result<Manga, AidokuError> {
        Err(AidokuError::new(1))
    }

    fn get_chapter_list(&self, _id: String) -> Result<Vec<Chapter>, AidokuError> {
        Err(AidokuError::new(1))
    }

    fn get_page_list(&self, _id: String) -> Result<Vec<Page>, AidokuError> {
        Err(AidokuError::new(1))
    }
}