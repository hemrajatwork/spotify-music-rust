// @generated automatically by Diesel CLI.

pub mod spotify_schema {
    diesel::table! {
        spotify_schema.backend_task (id) {
            id -> Int4,
            task_name -> Varchar,
            status -> Varchar,
            created_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        spotify_schema.song_information (song_id) {
            song_id -> Int4,
            artist -> Varchar,
            song -> Varchar,
            text -> Text,
            length -> Varchar,
            emotion -> Varchar,
            genre -> Varchar,
            album -> Varchar,
            release_date -> Varchar,
            key -> Varchar,
            tempo -> Float8,
            loudness -> Float8,
            time_signature -> Varchar,
            explicit -> Varchar,
            popularity -> Int4,
            energy -> Int4,
            danceability -> Int4,
            positiveness -> Int4,
            speechiness -> Int4,
            liveness -> Int4,
            acousticness -> Int4,
            instrumentalness -> Int4,
            good_for_party -> Bool,
            good_for_work -> Bool,
            good_for_relaxation -> Bool,
            good_for_exercise -> Bool,
            good_for_running -> Bool,
            good_for_yoga -> Bool,
            good_for_driving -> Bool,
            good_for_social_gatherings -> Bool,
            good_for_morning_routine -> Bool,
            similar_artist_1 -> Varchar,
            similar_song_1 -> Varchar,
            similarity_score_1 -> Float8,
            similar_artist_2 -> Varchar,
            similar_song_2 -> Varchar,
            similarity_score_2 -> Float8,
            similar_artist_3 -> Varchar,
            similar_song_3 -> Varchar,
            similarity_score_3 -> Float8,
            unique_id -> Varchar,
            created_at -> Nullable<Timestamp>,
            youtube_video -> Bool,
        }
    }

    diesel::table! {
        spotify_schema.song_youtube_detail (id) {
            id -> Int4,
            song_id -> Nullable<Int4>,
            youtube_link -> Varchar,
            created_at -> Nullable<Timestamp>,
        }
    }

    diesel::joinable!(song_youtube_detail -> song_information (song_id));

    diesel::allow_tables_to_appear_in_same_query!(
        backend_task,
        song_information,
        song_youtube_detail,
    );
}
