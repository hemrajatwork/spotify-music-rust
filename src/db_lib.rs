
use dotenvy::dotenv;
use diesel::prelude::*;
use std::env;
use std::error::Error;
use super::models::{BackendTask, SongInformation, SongYouTubeDetail, SongInformationBase};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use crate::schema::spotify_schema::{song_information::dsl::*, song_information, backend_task, song_youtube_detail, song_youtube_detail::dsl::*};
use log::{info, error, debug, warn};

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

pub struct CustomBool{
    value: Option<bool>,
}

impl Default for CustomBool {
    fn default() -> Self {
        CustomBool{value: Some(true)}
    }
}

pub fn fetch_song_youtube_data(conn: &mut PgConnection, limit:Option<i32>, offset:Option<i32>)-> Vec<(SongInformationBase, Option<SongYouTubeDetail>)>{
    let results = song_information::table
        .left_join(song_youtube_detail::table.on(song_information::song_id.eq(song_youtube_detail::song_id)))
        .select(((song_information::song_id, song_information::artist,
                  song_information::song), Option::<SongYouTubeDetail>::as_select())) // Select user and optional post
        .load::<(SongInformationBase, Option<SongYouTubeDetail>)>(conn)
        .expect("Error loading data");
    results
}
pub fn fetch_song_rows(conn: &mut PgConnection, song_ids:Option<&Vec<i32>>, limit:Option<i64>, has_youtube_link:Option<bool>) -> QueryResult<Vec<(i32, String, String, String)>>{
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

pub fn insert_task_records(conn: &mut PgConnection, info: &Vec<BackendTask>) -> QueryResult<usize> {
    let rows_inserted = diesel::insert_into(backend_task::table)
        .values(info)
        .execute(conn).unwrap();
    Ok(rows_inserted)
}

pub fn insert_song_youtube_detail(conn: &mut PgConnection, info: &Vec<SongYouTubeDetail>) -> QueryResult<usize> {
    info!("Inserting song youtube detail");
    let rows_inserted = diesel::insert_into(song_youtube_detail::table)
        .values(info)
        .execute(conn).unwrap();
    Ok(rows_inserted)
}

pub fn update_song_info(conn:&mut PgConnection, song_identifier:i32) {
    let updated_row = diesel::update(song_information::table.filter(song_information::song_id.eq(song_identifier)))
        .set(youtube_video.eq(true))
        .execute(conn);
    info!("updated row data {:?}", updated_row)
}