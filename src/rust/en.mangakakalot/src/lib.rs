
use aidoku::{
    prelude::*,
    std::{net::Request, html::Node, html::Html, String, Vec, error::AidokuError},
    manga::{Manga, MangaPageResult, Chapter, Page},
};

const BASE_URL: &str = "https://www.mangakakalot.gg";

#[register_source]
pub fn register() -> MangaSource {
    MangaSource::new(MangakakalotSource)
}

pub struct MangakakalotSource;

impl aidoku::std::net::RequestHeaders for MangakakalotSource {
    fn headers(&self) -> Vec<(String, String)> {
        Vec::new()
    }
}

impl aidoku::std::net::HttpSource for MangakakalotSource {
    fn get(&self, url: String) -> Result<String, AidokuError> {
        Request::new(url, self.headers()).html()
    }
}

impl aidoku::manga::Source for MangakakalotSource {
    fn get_manga_list(&self, _filters: Vec<aidoku::filter::Filter>, page: i32) -> Result<MangaPageResult, AidokuError> {
        let url = if page == 1 {
            BASE_URL.to_string()
        } else {
            format!("{}/?page={}", BASE_URL, page)
        };

        let html = Request::new(url, self.headers()).html()?;
        let document = Html::parse(html);

        let mut mangas: Vec<Manga> = Vec::new();

        for manga_node in document.select(".story_item a").array() {
            let manga_node = manga_node.as_node().unwrap();

            let title = manga_node.attr("title").unwrap_or_default();
            let href = manga_node.attr("href").unwrap_or_default();
            let cover = manga_node.select("img").first().attr("src").unwrap_or_default();

            mangas.push(Manga {
                id: href.to_string(),
                cover: cover.to_string(),
                title: title.to_string(),
                ..Default::default()
            });
        }

        Ok(MangaPageResult {
            manga: mangas,
            has_more: true,
        })
    }

    fn get_manga_details(&self, id: String) -> Result<Manga, AidokuError> {
        let url = format!("{}", id);
        let html = Request::new(url, self.headers()).html()?;
        let document = Html::parse(html);

        let title = document.select(".story-info-right h1").text().read();
        let cover = document.select(".info-image img").attr("src").unwrap_or_default();
        let description = document.select(".panel-story-info-description").text().read();
        let status_text = document.select(".story-info-right span").text().read();

        let status = if status_text.contains("Ongoing") {
            aidoku::manga::MangaStatus::Ongoing
        } else {
            aidoku::manga::MangaStatus::Completed
        };

        Ok(Manga {
            id,
            cover: cover.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            status,
            ..Default::default()
        })
    }

    fn get_chapter_list(&self, id: String) -> Result<Vec<Chapter>, AidokuError> {
        let url = format!("{}", id);
        let html = Request::new(url, self.headers()).html()?;
        let document = Html::parse(html);

        let mut chapters: Vec<Chapter> = Vec::new();

        for chapter_node in document.select(".panel-story-chapter-list a").array() {
            let chapter_node = chapter_node.as_node().unwrap();
            let chapter_title = chapter_node.text().read();
            let chapter_url = chapter_node.attr("href").unwrap_or_default();

            chapters.push(Chapter {
                id: chapter_url.to_string(),
                title: Some(chapter_title),
                ..Default::default()
            });
        }

        Ok(chapters)
    }

    fn get_page_list(&self, id: String) -> Result<Vec<Page>, AidokuError> {
        let url = format!("{}", id);
        let html = Request::new(url, self.headers()).html()?;
        let document = Html::parse(html);

        let mut pages: Vec<Page> = Vec::new();

        for (index, page_node) in document.select(".container-chapter-reader img").array().enumerate() {
            let page_node = page_node.as_node().unwrap();
            let img_url = page_node.attr("src").unwrap_or_default();

            pages.push(Page {
                index: index as i32,
                url: img_url.to_string(),
            });
        }

        Ok(pages)
    }
}
