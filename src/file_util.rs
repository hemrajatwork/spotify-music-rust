use std::error::Error;
//use std::fmt::Error;
use std::path::{PathBuf};
use csv;
use diesel::PgConnection;
use super::models::{SongInformation, UniqueId};
//use sha2::{Sha256};
//use std::thread;
//use diesel::PgConnection;
//use diesel::r2d2::ConnectionManager;
//use diesel::r2d2::Pool;
//use std::io::{Seek, SeekFrom};
//use std::fs;
use super::db_lib::{establish_connection, insert_song_records};

pub fn read_file<'a>(file_path:PathBuf, _has_header:bool) ->Result<(), Box<dyn Error>>{
    let mut rdr = csv::Reader::from_path(file_path)?;
    let values = rdr.deserialize();

    let batch_size:usize = 500;
    let mut pg_connection:PgConnection =  establish_connection();
    let mut rows:Vec<SongInformation> = Vec::new();
    for (line_num, line) in values.enumerate() {
        
        if line_num == 0 {
            continue;
        }

        println!("Reading line number: {}", line_num);
        let mut record: SongInformation = line?;
        println!(" song record: {}", record.artist);
        record.unique_id = record.get_unique_id();
        //println!("{:?}", record);
        rows.push(record);
        if rows.len() == batch_size{
            insert_song_records(&mut pg_connection, &rows)?;
        }
        
    }
    if !rows.is_empty(){
        insert_song_records(&mut pg_connection, &rows)?;
    }
    Ok(())
}

pub fn get_file_total_line_count(file_path:PathBuf, _has_header:bool) ->usize{
    let mut rdr = csv::Reader::from_path(file_path).unwrap();
    let mut no_lines = 0;
    for (line_num,_line) in rdr.records().enumerate() {
        println!("{}",line_num);
        no_lines = line_num;
    }
    no_lines
}
