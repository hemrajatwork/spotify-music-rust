use diesel::QueryResult;
use super::youtube::{search_youtube, parse_youtube_res};
use reqwest;
use super::db_lib::{establish_connection, fetch_song_rows};
use super::constant::{YOUTUBE_TOKEN_LIMIT, YOUTUBE_LIST_API_COST};
use rocket::tokio::{task, self};
use futures::future::join_all;


#[tokio::main]
async fn main(){
    let mut pg_conn = establish_connection();
    let limit: i32 = YOUTUBE_TOKEN_LIMIT/YOUTUBE_LIST_API_COST;
    let result: QueryResult<Vec<(i32, String, String, String)>> =  fetch_song_rows(& mut pg_conn, None, Some(5));

    match result {
        Ok(song_info) => {
            let mut search_text_list: Vec<String> = vec![];

            let mut futures: Vec<_> = vec![];
            for row in song_info {
                let search_text:String = row.1;
                futures.push(search_youtube(search_text, 1))
            };
            let results = join_all(futures).await;

            for (i, result) in results.into_iter().enumerate() {
                match result {
                    Ok(data) => println!("Data from URL {}: {}", i + 1, data),
                    Err(e) => eprintln!("Error fetching data from URL {}: {}", i + 1, e),
                }
            }

            println!("All threads finished.");
        }
        Err(e) => {}
    }

}




