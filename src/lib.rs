use std::error::Error;
use std::io::copy;
use std::fs::File;
use std::env;
use std::io::Cursor;
use regex::Regex;
use reqwest::header::{HeaderMap, DNT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT, ACCEPT, REFERER};
use serde::Deserialize;

use std::collections::HashMap;

const URL: &str = "https://duckduckgo.com/";

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    height: u32,
    width: u32,
    image: String,
    source: String,
    thumbnail: String,
    title: String,
    url: String
}

#[derive(Deserialize)]
pub struct Response {
    ads: Option<String>,
    next: String,
    query: String,
    #[serde(rename = "queryEncoded")]
    query_encoded: String,
    response_type: String,
    pub results: Vec<SearchResult>,
    vqd: HashMap<String, String>
}

pub async fn find_images(search_term: &str, token: &str) -> Result<Vec<String>, Box<dyn Error>>{
    let mut headers = HeaderMap::new();
    headers.insert(DNT, "1".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, sdch".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en-GB,en-US;q=0.8,en;q=0.6,ms;q=0.4".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
    headers.insert(ACCEPT, "application/json, text/javascript, */*; q=0.01".parse().unwrap());
    headers.insert(REFERER, "https://duckduckgo.com".parse().unwrap());
    headers.insert("authority", "duckduckgo.com".parse().unwrap());

    let client = reqwest::Client::new();
    let json: Response = client
        .get(format!("{}i.js", URL))
        .headers(headers)
        .query(&[
            ("q", search_term),
            ("l", "wt-wt"),
            ("o", "json"),
            ("vqd", token),
            ("f", ",,,"),
            ("p", "1")
        ])
        .send()
        .await?
        .json::<Response>()
        .await?;

    let images : Vec<String> = json.results.iter()
        .map(|result| result.image.clone()).collect();
    println!("Image: {}", images[0]);
    Ok(images)
}

pub async fn get_token(search_term: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let text = client
        .get("https://duckduckgo.com/")
        .query(&[("q", search_term)])
        .send()
        .await?
        .text()
        .await?;

    let re = Regex::new(r"vqd=([\d-]+)&").unwrap();
    let caps = re.captures(&text).unwrap();
    let token = caps.get(1).unwrap().as_str();

    Ok((String::from(token)))
}

pub async fn download_image(url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        let fname = env::current_dir().unwrap().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    println!("Dest {:?}", dest);
    let mut content = Cursor::new(response.bytes().await?);
    // println!("Content length: {}", content.len());
    match copy(&mut content, &mut dest) {
        Err(e) => println!("Error {}", e),
        Ok(bytes) => println!("Copied successfully {} bytes", bytes)
    };
    Ok(())
}