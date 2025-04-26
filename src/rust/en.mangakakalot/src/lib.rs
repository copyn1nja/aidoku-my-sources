use aidoku::{
    prelude::*,
    std::{net::Request, html::Html, String, Vec, error::AidokuError},
    manga::{Manga, MangaPageResult, Chapter, Page, MangaStatus},
};

const BASE_URL: &str = "https://www.mangakakalot.gg";

#[register_source]
pub fn register() -> MangaSource {
    MangaSource::new(MangakakalotSource)
}

pub struct MangakakalotSource;

impl aidoku::manga::Source for MangakakalotSource {
    fn get_manga_list(&self, _filters: Vec<aidoku::filter::Filter>, _page: i32) -> Result<MangaPageResult, AidokuError> {
        let html = Request::new(BASE_URL, Vec::new()).html()?;
        let document = Html::parse(html);

        let mut mangas = Vec::new();

        for manga in document.select(".story_item").array().take(10) {
            let node = manga.as_node().unwrap();
            let title = node.select("h3 a").text().read();
            let cover = node.select("img").attr("src").unwrap_or_default();
            let id = node.select("h3 a").attr("href").unwrap_or_default();

            mangas.push(Manga {
                id: id.to_string(),
                cover: cover.to_string(),
                title: title.to_string(),
                author: String::new(),
                artist: String::new(),
                description: String::new(),
                url: id.to_string(),
                categories: Vec::new(),
                status: MangaStatus::Unknown,
                nsfw: false,
            });
        }

        Ok(MangaPageResult {
            manga: mangas,
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