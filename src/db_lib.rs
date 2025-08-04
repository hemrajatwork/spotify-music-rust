
use dotenvy::dotenv;
use diesel::prelude::*;
use std::env;
use diesel::dsl::Select;
use crate::schema::song_information;
use super::models::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
//use diesel::result::Error;
//use std::thread;
//use crate::schema;
use crate::schema::song_information::dsl::*;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_song_records(conn: &mut PgConnection, song_info: &Vec<SongInformation>) -> QueryResult<usize> {
    let rows_inserted = diesel::insert_into(song_information::table)
        .values(song_info)
        .execute(conn).unwrap();
    Ok(rows_inserted)
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>>{
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .max_size(15)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn fetch_song_rows(conn: &mut PgConnection, song_ids:Option<&Vec<i32>>, limit:Option<i64>) -> QueryResult<Vec<(i32, String, String, String)>>{
    /*let mut query = song_information::table.into_boxed();*/
    let mut query = song_information.select((song_id, song, artist, emotion)).into_boxed();

    match song_ids {
        Some(selected_song_ids )=>{
            query = query.filter(song_information::song_id.eq_any(selected_song_ids));
        },
        None => {}
    }
    match limit {
        Some(limit_records) => {
            query = query.limit(limit_records)
        },
        None => {}
    }
    let result = query.load::<(i32, String, String, String)>(conn);
    result
}

