#[path = "../constant.rs"] mod constant;
use constant::{YOUTUBE_API_URL, YOUTUBE_API_KEY};
use serde_json::Value;
use urlencoding;
use rocket::serde::json::Json;
use reqwest;
use rocket::Error;
use rocket::form::validate::Len;

pub async fn search_youtube<'a>(search_text:& 'a str, result_limit:i32) ->Result<String, String>{
    println!("start youtube search");
    let search = format!("{} song", search_text);
    let search_str = &*search;
    let url = format!(
        "{}/search?part=snippet&q={}&maxResults={}&order=title&key={}&type=video&videoDuration=short&videoEmbeddable=true",
        YOUTUBE_API_URL, urlencoding::encode(search_str), result_limit, YOUTUBE_API_KEY);
    println!("youtube search url {}",url);
    match reqwest::get(url).await{
        Ok(data) => Ok(data.text().await.unwrap()),
        Err(e) => Err(e.to_string())
    }
}
pub fn parse_youtube_res<'a>(api_response:& 'a str, search_term:& 'a str)-> Option<String>{
    let api_response_json:Value = serde_json::from_str(api_response).expect("json parse error");
    if let Some(obj) = api_response_json.as_object() {
        for val in obj["items"].as_array().unwrap() {
            /*let etag = val.get("etag");*/
            for (item_index, item_val) in val.as_object().iter().enumerate() {
                println!("{:?}\n**", item_val);
                let video_id = item_val["id"]["videoId"].as_str().unwrap_or("video id not found");
                println!("found video id {}", video_id);
                let title = item_val["snippet"]["title"].as_str().expect("title not found");
                if title.contains(search_term){
                    return Some(video_id.to_string());
                } else if item_index ==item_val.len() - 1{
                    return Some(video_id.to_string());
                }
            }
        }
    }
    None
}

/*
{"etag":"UuLYyCrbY17vrISb-FnH1ZBLH7E","id":{"kind":"youtube#video","videoId":"198aCDYc79I"},"kind":"youtube#searchResult","snippet":{"channelId":"UC6Eu8LXVDUGG2SgrFj9YQVQ","channelTitle":"Osi ","description":"Undertale - last breath | phase 4 | battle animation На русском языке! Перевод - я Анимация- Corey animations хэштеги ...","liveBroadcastContent":"none","publishTime":"2025-04-25T20:00:01Z","publishedAt":"2025-04-25T20:00:01Z","thumbnails":{"default":{"height":90,"url":"https://i.ytimg.com/vi/198aCDYc79I/default.jpg","width":120},"high":{"height":360,"url":"https://i.ytimg.com/vi/198aCDYc79I/hqdefault.jpg","width":480},"medium":{"height":180,"url":"https://i.ytimg.com/vi/198aCDYc79I/mqdefault.jpg","width":320}},"title":"Полная версия на канале! Undertale - last breath | phase 4 | battle animation На русском языке!"}
 */