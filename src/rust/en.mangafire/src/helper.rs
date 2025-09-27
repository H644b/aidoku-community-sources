use aidoku::prelude::*;
use aidoku::std::net::Request;
use aidoku::std::String;

pub fn get(url: &str) -> String {
    let resp = Request::new(url, Method::Get).html().expect("Request failed");
    resp.html().read()
}

pub fn urlencode(input: &str) -> String {
    percent_encoding::utf8_percent_encode(input, percent_encoding::NON_ALPHANUMERIC).to_string()
}
