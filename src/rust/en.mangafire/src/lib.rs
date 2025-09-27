#![no_std]
extern crate alloc;

use aidoku::prelude::*;
use aidoku::AidokuError;
use aidoku::AidokuErrorKind;
use aidoku::ContentRating;
use aidoku::DeepLinkResult;
use aidoku::Filter;
use aidoku::FilterKind;
use aidoku::Manga;
use aidoku::MangaPageResult;
use aidoku::MangaStatus;
use aidoku::Viewer;
use aidoku::Chapter;
use aidoku::Page;
use aidoku::imports::net::HttpMethod;
use aidoku::imports::net::Request;
use aidoku::imports::String;
use aidoku::imports::Vec;
use aidoku::imports::html::Html;

mod helper;
use helper::*;

const BASE_URL: &str = "https://mangafire.to";

struct MangaFire;

impl Source for MangaFire {
    fn get_manga_list(&self, filters: Vec<Filter>, page: i32) -> Result<MangaPageResult, AidokuError> {
        let mut url = String::from(BASE_URL) + "/filter";
        let mut queries: Vec<String> = Vec::new();

        for filter in filters {
            if let FilterKind::Select = filter.kind {
                if let Some(value) = filter.value.as_string() {
                    if value == "Any" { continue; }
                    let key = filter.name.unwrap_or_default();
                    match key.as_str() {
                        "genre" => {
                            queries.push(format!("genre={}", urlencode(&value.to_lowercase().replace(" ", "-"))));
                        }
                        "type" => {
                            queries.push(format!("type={}", value.to_lowercase()));
                        }
                        "status" => {
                            queries.push(format!("status={}", value.to_lowercase().replace(' ', "-")));
                        }
                        "language" => {
                            let code = match value.as_str() {
                                "English" => "en",
                                "French" => "fr",
                                "Spanish" => "es",
                                "Spanish (LATAM)" => "es-419",
                                "Portuguese" => "pt",
                                "Portuguese (BR)" => "pt-br",
                                "Japanese" => "ja",
                                _ => continue,
                            };
                            queries.push(format!("lang={}", code));
                        }
                        "year" => {
                            if value != "Any" {
                                queries.push(format!("year={}", value));
                            }
                        }
                        "length" => {
                            if value.starts_with(">=") {
                                let num = value.trim_start_matches(">=").trim();
                                queries.push(format!("chapters=>{}", num));
                            }
                        }
                        "sort" => {
                            let mapped = match value.as_str() {
                                "Newest" => "new",
                                "Updated" => "updated",
                                "Added" => "added",
                                "Random" => "random",
                                _ => continue,
                            };
                            queries.push(format!("sort={}", mapped));
                        }
                        _ => {}
                    }
                }
            }
        }

        queries.push(format!("page={}", page));
        url.push('?');
        url.push_str(&queries.join("&"));

        let html = Request::new(&url, HttpMethod::Get).html()?;
        parse_manga_list(html)
    }

    fn get_manga_details(&self, id: String) -> Result<Manga, AidokuError> {
        let url = format!("{}/manga/{}", BASE_URL, id);
        let html = Request::new(&url, HttpMethod::Get).html()?;
        parse_manga_details(html, id)
    }

    fn get_chapter_list(&self, id: String) -> Result<Vec<Chapter>, AidokuError> {
        let url = format!("{}/manga/{}/chapters", BASE_URL, id);
        let html = Request::new(&url, HttpMethod::Get).html()?;
        parse_chapter_list(html)
    }

    fn get_page_list(&self, _manga_id: String, chapter_id: String) -> Result<Vec<Page>, AidokuError> {
        let url = format!("{}/read/{}", BASE_URL, chapter_id);
        let html = Request::new(&url, HttpMethod::Get).html()?;
        parse_page_list(html)
    }
}

impl DeepLinkHandler for MangaFire {
    fn handle_url(&self, url: String) -> Result<DeepLinkResult, AidokuError> {
        if url.contains("/manga/") {
            let id = url.split("/manga/").nth(1).unwrap_or("").to_string();
            let manga = self.get_manga_details(id)?;
            Ok(DeepLinkResult::Manga(manga))
        } else {
            Err(AidokuError {
                reason: AidokuErrorKind::Unimplemented,
            })
        }
    }
}

register_source!(MangaFire);