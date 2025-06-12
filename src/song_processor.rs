use std::path::PathBuf;
use super::file_util;
use super::db_lib;
use std::error::Error;

use db_lib::{establish_connection, insert_song_records};

/*pub(crate) fn handle_song_data<'a>(filepath:PathBuf) -> Result<&'a str, Box<dyn Error>>{
    let res = file_util::read_file(filepath, true);
    if res.is_err(){
        return Err(res.unwrap_err());
    }else{
        let song_info = res.unwrap();
        let mut pg_conn = establish_connection();
        let _insert_data = insert_song_records(&mut pg_conn, &song_info);
        return Ok(From::from("successfully inserted song records into DB "))
    }
    
}*/
