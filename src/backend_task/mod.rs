use diesel::QueryResult;
use super::youtube::{search_youtube, parse_youtube_res};
use reqwest;
use super::db_lib::{establish_connection, fetch_song_rows, insert_task_records,
                    update_song_info, insert_song_youtube_detail};
use super::constant::{YOUTUBE_TOKEN_LIMIT, YOUTUBE_LIST_API_COST, FAIL, SUCCESS};
use rocket::tokio::{task, self};
use futures::future::join_all;
use super::models::{SongYouTubeDetail, BackendTask};
use log::{info, error, debug, warn};


pub async fn BackEndTask(){
    let mut pg_conn = establish_connection();
    let youtube_api_limit: i32 = YOUTUBE_TOKEN_LIMIT/YOUTUBE_LIST_API_COST;
    info!("fetching songs from database");
    let result: QueryResult<Vec<(i32, String, String, String)>> =  fetch_song_rows(& mut pg_conn, None, Some(2), None, Some(false));
    info!("finished fetching songs from database");
    let task_name = "spotify_task".to_string();
    let mut failure_flg = false;
    let mut task_status = SUCCESS.to_string();
    let mut task_results:Vec<BackendTask>= vec![];
    let mut song_youtube_detail:Vec<SongYouTubeDetail> = vec![];

    match result {
        Ok(song_info) => {
            let mut search_text_list: Vec<String> = vec![];
            let mut song_ids = vec![];
            let mut futures: Vec<_> = vec![];
            for row in song_info {
                let search_text:String = row.1;
                info!("search text: {} , song_id: {}", search_text, row.0);
                song_ids.push(row.0);
                futures.push(search_youtube(search_text, 1))
            };
            let results = join_all(futures).await;
            let mut pg_conn = establish_connection();
            for (i, api_response) in results.into_iter().enumerate() {

                match api_response {
                    Ok(data) => {
                        let video_id:Option<String> = parse_youtube_res(&data.0, &data.1);
                        match video_id {
                            Some(video_id)=>{
                                debug!("song_id {},search_text {}, video_id {}", song_ids[i], data.1, video_id);
                                song_youtube_detail.push(
                                    SongYouTubeDetail{id:None, song_id:song_ids[i], youtube_link: video_id, created_at:None});
                                update_song_info(& mut pg_conn, song_ids[i])
                            }
                            None => {
                                info!("Not found video id for search_text {}", data.1);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error fetching data from search term {}: {}", i + 1, e);
                        failure_flg = true;
                    },
                }
            }

            info!("All threads finished.");
            if failure_flg {
                task_status = FAIL.to_string();
            }
        }
        Err(e) => {
            task_status = FAIL.to_string();
            error!("error processing thread {:?}", e)
        }
    }
    task_results.push(BackendTask{task_name:task_name, status:task_status});
    insert_task_records(& mut pg_conn, &task_results);
    insert_song_youtube_detail(& mut pg_conn, &song_youtube_detail);
}




