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
mod diesel_apply_migration;
mod response_message;
use crate::file_util::{read_file, get_song_data, get_song_list_with_video, get_unique_count};
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
use crate::db_lib::{establish_connection, YoutubeData};
use diesel_apply_migration::{run_migrations};



#[get("/parse_csv_data")]
fn parse_csv_data() {
    let song_csv_file_name:PathBuf = PathBuf::from("./data_file/spotify_dataset.csv");
    match read_file(song_csv_file_name, true){
        Ok(song_data) => {
            println!("Song Data inserted successfully");
            
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
        
    }
    get_song_data(None, Some(10), None);
}
#[get("/<song_id>")]
fn get_song(song_id: i32) -> Json<Vec<(i32, String, String, String)>> {
    let mut song_ids:Vec<i32> = Vec::new();
    song_ids.push(song_id);
    let data = get_song_data(Some(&song_ids), Some(5), None).unwrap();
    Json(data)
}

#[get("/all?<offset>&<limit>")]
fn get_all_songs(offset:Option<usize>, limit:Option<usize>) -> Json<Vec<(i32, String, String, String)>> {
    let data = get_song_data(None, Some(100), None).unwrap();
    Json(data)
}

#[get("/home?<offset>&<limit>")]
fn home(offset:i64, limit:i64) -> Template {
    let data = get_song_list_with_video(limit, offset);
    let result = get_unique_count();
    let mut total_song:i64 = result.0;
    let mut total_youtube:i64 = result.3;
    let mut total_artist:i64 = result.1;
    let mut total_album:i64 = result.2;
    Template::render ("home", context! {data: data, pagename: "Home",
        song_total: total_song, total_artist: total_artist, total_album: total_album,
        total_youtube_link: total_youtube,
    pagedetail: "Spotify Music Data"})
}
#[get("/rocket_framework")]
fn get_rocket_framework_code() -> Template{
    Template::render("rocket_framework", context!{pagename: "Rocket Framework",
        pagedetail: "code rocket framework"})
}

#[get("/csv_parser")]
fn get_csv_parser_code() -> Template{
    Template::render("csv_parser", context!{pagename: "CSV Parser",
        pagedetail: "code csv parser"})
}

#[get("/postgres_storage")]
fn get_postgres_storage_code() -> Template{
    Template::render("postgres_storage", context!{pagename: "Postgres Storage",
        pagedetail: "code postgres storage"})
}

#[get("/youtube_api")]
fn get_youtube_api_code() -> Template{
    Template::render("youtube_api", context!{pagename: "Youtube API",
        pagedetail: "Code youtube api"})
}

#[get("/threading")]
fn get_threading_code() -> Template{
    Template::render("threading", context!{pagename: "Threading & Async",
        pagedetail: "threading & async"})
}

#[get("/resume")]
fn get_resume() -> Template{
    Template::render("resume", context!{pagename: "Resume",
        pagedetail: "resume"})
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
    log4rs::init_file("/home/ec2-user/spotify-music-rust/config/log_setting.yaml", Default::default()).unwrap();
    info!("Rocket application starting...");
    match (run_migrations()){
      Ok(migration_sql)=>{
          info!("Running migration...");
      }, Err(error)=>{
            error!("{}", error);
        }
    };

    rocket::build()
        .mount("/song", routes![get_song])
        .mount("/", routes![home])
        .mount("/", routes![parse_csv_data])
        .mount("/", routes![get_rocket_framework_code] )
        .mount("/", routes![get_csv_parser_code] )
        .mount("/", routes![get_postgres_storage_code] )
        .mount("/", routes![get_threading_code] )
        .mount("/", routes![get_resume] )
        .mount("/song", routes![get_all_songs])
        .mount("/song", routes![get_youtube_video])
        .mount("/song", routes![user_search])
        .mount("/", routes![fetch_youtube_link])
        .mount("/", FileServer::from("/home/ec2-user/spotify-music-rust/static/"))
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}