#![no_std]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use aidoku::imports::net::{HttpMethod, Request};
use aidoku::prelude::encode_uri_component;
use aidoku::prelude::{Manga, Chapter, Page};

pub fn get(url: &str) -> String {
    Request::new(url, HttpMethod::Get).html().expect("Request failed").read()
}

pub fn urlencode(input: &str) -> String {
    encode_uri_component(input)
}

// ----- Parser stubs -----
pub fn parse_manga_list(_html: String) -> Result<Vec<Manga>> {
    Ok(Vec::new())
}

pub fn parse_manga_details(_html: String, _id: String) -> Result<Manga> {
    Ok(Manga::new())
}

pub fn parse_chapter_list(_html: String) -> Result<Vec<Chapter>> {
    Ok(Vec::new())
}

pub fn parse_page_list(_html: String) -> Result<Vec<Page>> {
    Ok(Vec::new())
}
