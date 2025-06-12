use std::path::PathBuf;
mod song_processor;
mod file_util;
mod models;
mod schema;
mod db_lib;

use diesel::prelude::*;
//use song_processor::handle_song_data;
use crate::file_util::read_file;

fn main() {
    let song_csv_file_name:PathBuf = PathBuf::from("./data_file/spotify_dataset.csv");
    match read_file(song_csv_file_name, true){
        Ok(song_data) => {
            println!("Song Data inserted successfully");
            
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
        
    }
}
