#[macro_use] extern crate rocket;
use std::path::PathBuf;
mod song_processor;
mod file_util;
mod models;
mod schema;
mod db_lib;
mod utils;
use diesel::prelude::*;
//use song_processor::handle_song_data;
use crate::file_util::{read_file, get_song_data};
use crate::models::{SongInformation, SongInformationBase};
use rocket::serde::json::Json;

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
#[get("/song/<song_id>")]
fn get_song(song_id: i32) -> Json<Vec<(i32, String, String, String)>> {
    let mut song_ids:Vec<i32> = Vec::new();
    song_ids.push(song_id);
    let data = get_song_data(Some(&song_ids), Some(5)).unwrap();
    Json(data)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_song])
}