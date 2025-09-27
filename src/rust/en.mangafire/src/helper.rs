#![no_std]
extern crate alloc;

use alloc::string::String;
use aidoku::imports::net::{HttpMethod, Request};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub fn get(url: &str) -> String {
    Request::new(url, HttpMethod::Get).html().expect("Request failed").read()
}

pub fn urlencode(input: &str) -> String {
    utf8_percent_encode(input, NON_ALPHANUMERIC).to_string()
}
