#[macro_use] extern crate rocket;
use std::path::PathBuf;
mod song_processor;
mod file_util;
mod models;
mod schema;
mod db_lib;
mod utils;
use diesel::prelude::*;
mod youtube;
mod constant;

mod response_message;
//use song_processor::handle_song_data;
use crate::file_util::{read_file, get_song_data};
use crate::models::{SongInformation, SongInformationBase};
use rocket::serde::json::Json;
use youtube::search_youtube;
use response_message::{APIResponse};
use serde_json::{self, Value};

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
    let res = search_youtube(search_text, limit).await;
    println!("{:?}", res);
    match res{
        Ok(data) =>
            {
                let json_data:Value = serde_json::from_str(data.as_str()).unwrap();
                if &json_data.get("error").is_some() == &true {
                    Json(APIResponse { msg: "please try again later".to_string() , code:400})
                } else {
                    Json(APIResponse{msg:json_data.to_string(), code:200})
                }

            },
        Err(e) => Json(APIResponse{msg:e.to_string(), code:500})
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/song", routes![get_song])
        .mount("/song", routes![get_all_songs])
        .mount("/song", routes![get_youtube_video])
}