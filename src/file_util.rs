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
use std::time::Instant;

pub fn read_file<'a>(file_path:PathBuf, _has_header:bool) ->Result<(), Box<dyn Error>>{
    let mut rdr = csv::Reader::from_path(file_path)?;
    let values = rdr.deserialize();

    let start_insert = Instant::now();
    let batch_size:usize = 500;
    let mut pg_connection:PgConnection =  establish_connection();
    let mut rows:Vec<SongInformation> = Vec::with_capacity(batch_size);
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
            rows.clear();
        }
        
    }
    if !rows.is_empty(){
        insert_song_records(&mut pg_connection, &rows)?;
    }
    let duration = start_insert.elapsed();
    println!("Elapsed time: {:?}", duration);
    println!("Elapsed seconds: {:.2}", duration.as_secs());
    Ok(())
}

pub fn fetch_rows(){
    
}

pub fn clean_data(){
    
}


