#![no_std]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use aidoku::{
    imports::net::{HttpMethod, Request},
    prelude::*,
    Chapter, Filter, Manga, Page,
};

mod helper;
use helper::*;

const BASE_URL: &str = "https://mangafire.to";

pub fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<Vec<Manga>> {
    let mut url = format!("{}/filter", BASE_URL);
    let mut queries: Vec<String> = Vec::new();

    for filter in filters {
        if let Filter::Select { key, value, .. } = filter {
            let val = value.unwrap_or_default();
            if val == "Any" { continue; }
            match key.as_str() {
                "genre" => queries.push(format!("genre={}", val.to_lowercase().replace(' ', "-"))),
                "type" => queries.push(format!("type={}", val.to_lowercase())),
                "status" => queries.push(format!("status={}", val.to_lowercase().replace(' ', "-"))),
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
                "year" => if val != "Any" { queries.push(format!("year={}", val)) },
                "length" => if val.starts_with(">=") {
                    let num = val.trim_start_matches(">=").trim();
                    queries.push(format!("chapters=>{}", num));
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

    queries.push(format!("page={}", page));
    url.push('?');
    url.push_str(&queries.join("&"));

    let html = Request::new(&url, HttpMethod::Get).html()?;
    parse_manga_list(html)
}

pub fn get_manga_details(id: String) -> Result<Manga> {
    let url = format!("{}/manga/{}", BASE_URL, id);
    let html = Request::new(&url, HttpMethod::Get).html()?;
    parse_manga_details(html, id)
}

pub fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
    let url = format!("{}/manga/{}/chapters", BASE_URL, id);
    let html = Request::new(&url, HttpMethod::Get).html()?;
    parse_chapter_list(html)
}

pub fn get_page_list(_manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
    let url = format!("{}/read/{}", BASE_URL, chapter_id);
    let html = Request::new(&url, HttpMethod::Get).html()?;
    parse_page_list(html)
}

pub fn handle_url(url: String) -> Option<Manga> {
    if url.contains("/manga/") {
        let id = url.split("/manga/").nth(1).unwrap_or("").to_string();
        get_manga_details(id).ok()
    } else {
        None
    }
}
