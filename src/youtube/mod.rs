#[path = "../constant.rs"] mod constant;
use constant::{YOUTUBE_API_URL, YOUTUBE_API_KEY};
use serde_json;
use urlencoding;
use rocket::serde::json::Json;
use reqwest;
use rocket::Error;

pub async fn search_youtube<'a>(search_text:& 'a str,result_limit:i32)->Result<String, String>{
    println!("start youtube search");
    let url = format!(
        "{}/search?part=snippet&q={}&maxResults={}&order=title&key={}&type=video",
        YOUTUBE_API_URL, urlencoding::encode(search_text), result_limit, YOUTUBE_API_KEY);
    println!("youtube search url {}",url);
    match reqwest::get(url).await{
        Ok(data) => Ok(data.text().await.unwrap()),
        Err(e) => Err(e.to_string())
    }
}