#![no_std]
extern crate alloc;

use aidoku::prelude::*;
use aidoku::AidokuError;
use aidoku::error::AidokuErrorKind;
use aidoku::MangaContentRating;
use aidoku::DeepLink;
use aidoku::Filter;
use aidoku::FilterType;
use aidoku::Manga;
use aidoku::MangaPageResult;
use aidoku::MangaStatus;
use aidoku::MangaViewer;
use aidoku::Chapter;
use aidoku::Page;
use aidoku::imports::net::HttpMethod;
use aidoku::imports::net::Request;
use aidoku::alloc::String;
use aidoku::alloc::Vec;
use aidoku::imports::html::Html;
use aidoku::Source;
use aidoku::DeepLinkHandler;

mod helper;
use helper::*;

const BASE_URL: &str = "https://mangafire.to";

struct MangaFire;

impl Source for MangaFire {
    fn get_manga_list(&self, filters: Vec<Filter>, page: i32) -> Result<MangaPageResult, AidokuError> {
        let mut url = String::from(BASE_URL) + "/filter";
        let mut queries: Vec<String> = Vec::new();

        for filter in filters {
            match filter.kind {
                FilterType::Select => {
                    let key = filter.name;
                    if let Ok(val) = filter.value.as_string() {
                        if val == "Any" { continue; }
                        match key.as_str() {
                            "genre" => {
                                queries.push(format!(
                                    "genre={}",
                                    urlencode(&val.to_lowercase().replace(" ", "-"))
                                ));
                            }
                            "type" => {
                                queries.push(format!("type={}", val.to_lowercase()));
                            }
                            "status" => {
                                queries.push(format!("status={}", val.to_lowercase().replace(' ', "-")));
                            }
                            "language" => {
                                let code = match val.as_str() {
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
                                if val != "Any" {
                                    queries.push(format!("year={}", val));
                                }
                            }
                            "length" => {
                                if val.starts_with(">=") {
                                    let num = val.trim_start_matches(">=").trim();
                                    queries.push(format!("chapters=>{}", num));
                                }
                            }
                            "sort" => {
                                let mapped = match val.as_str() {
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
                _ => {}
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
    fn handle_url(&self, url: String) -> Result<DeepLink, AidokuError> {
        if url.contains("/manga/") {
            let id = url.split("/manga/").nth(1).unwrap_or("").to_string();
            let manga = self.get_manga_details(id)?;
            Ok(DeepLink {
                manga: Some(manga),
                chapter: None,
            })
        } else {
            Err(AidokuError {
                reason: AidokuErrorKind::Unimplemented,
            })
        }
    }
}