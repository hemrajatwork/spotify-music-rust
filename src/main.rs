#[macro_use] extern crate rocket;
use log::{info, error, debug, warn};
use log4rs;

use std::path::PathBuf;
mod file_util;
mod models;
mod schema;
mod db_lib;
mod utils;
use diesel::prelude::*;
mod youtube;
mod constant;
mod form_struct;
mod backend_task;
mod response_message;
use crate::file_util::{read_file, get_song_data};
use crate::models::{SongInformation, SongInformationBase};
use rocket::serde::json::Json;
use youtube::{search_youtube, parse_youtube_res};
use response_message::{APIResponse};
use serde_json::{self, Value};
use form_struct::{UserSearch};
use rocket::form::{Form};
use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;
use backend_task::{BackEndTask};

#[get("/")]
fn index() -> Template {
    Template::render("home", context! { message: "Hello from Tera!" })
}
/*fn main() {
    let song_csv_file_name:PathBuf = PathBuf::from("./data_file/spotify_dataset.csv");
    /*match read_file(song_csv_file_name, true){
        Ok(song_data) => {
            println!("Song Data inserted successfully");
            
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
        
    }*/
    get_song_data();
}*/
#[get("/<song_id>")]
fn get_song(song_id: i32) -> Json<Vec<(i32, String, String, String)>> {
    let mut song_ids:Vec<i32> = Vec::new();
    song_ids.push(song_id);
    let data = get_song_data(Some(&song_ids), Some(5)).unwrap();
    Json(data)
}

#[get("/all")]
fn get_all_songs() -> Json<Vec<(i32, String, String, String)>> {
    let data = get_song_data(None, Some(100)).unwrap();
    Json(data)
}
#[get("/search/<search_text>/<limit>")]
async fn get_youtube_video(search_text:&str, limit:i32)->Json<APIResponse<String>>{
    let res = search_youtube(search_text.to_string(), limit).await;
    match res{
        Ok(data) =>
            {
                let json_data:Value = serde_json::from_str(data.0.as_str()).unwrap();
                if &json_data.get("error").is_some() == &true {
                    Json(APIResponse { msg: "please try again later".to_string() , code:400})
                } else {
                    let video_id = parse_youtube_res(&json_data.to_string(), &search_text).unwrap();
                    let youtube_link = format!("https://www.youtube.com/watch?v={}", video_id);
                    println!("youtube link {}", youtube_link);

                    Json(APIResponse{msg:youtube_link, code:200})
                }

            },
        Err(e) => Json(APIResponse{msg:e.to_string(), code:500})
    }
}

#[get("/youtube/link")]
async fn fetch_youtube_link(){
    BackEndTask().await;
}

#[post("/user_search", data="<search_data>")]
fn user_search(search_data: Form<UserSearch<'_>>) -> Json<APIResponse<String>>{
    let search_text = search_data.search_text.to_string();
    Json(APIResponse{msg:search_text, code: 200})
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    log4rs::init_file("./config/log_setting.yaml", Default::default()).unwrap();
    info!("Rocket application starting...");

    rocket::build()
        .mount("/song", routes![get_song])
        .mount("/song", routes![get_all_songs])
        .mount("/song", routes![get_youtube_video])
        .mount("/song", routes![user_search])
        .mount("/", routes![index])
        .mount("/", routes![fetch_youtube_link])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}