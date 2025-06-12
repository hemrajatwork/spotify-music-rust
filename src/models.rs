use serde::{Deserialize, Serialize, Deserializer, Serializer};
use diesel::prelude::*;
use std::time::SystemTime;
use sha2::{Digest, Sha256};
use crate::schema::song_information;

pub trait UniqueId{
    fn get_unique_id(&self) -> String;
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = song_information)]
pub struct SongInformation {
    #[serde(alias = "Artist(s)")]
    pub artist: String,
    pub song: String,
    pub text: String,
    #[serde(alias = "Length")]
    pub length: String,
    pub emotion: String,
    #[serde(alias = "Genre")]
    pub genre: String,
    #[serde(alias = "Album")]
    pub album: String,
    #[serde(alias = "Release Date")]
    pub release_date: String,
    #[serde(alias = "Key")]
    pub key: String,
    #[serde(alias = "Tempo")]
    pub tempo: f64,
    #[serde(alias = "Loudness (db)")]
    pub loudness: f64,
    #[serde(alias = "Time signature")]
    pub time_signature: String,
    #[serde(alias = "Explicit")]
    pub explicit: String,
    #[serde(alias = "Popularity")]
    pub popularity: i32,
    #[serde(alias = "Energy")]
    pub energy: i32,
    #[serde(alias = "Danceability")]
    pub danceability: i32,
    #[serde(alias = "Positiveness")]
    pub positiveness: i32,
    #[serde(alias = "Speechiness")]
    pub speechiness: i32,
    #[serde(alias = "Liveness")]
    pub liveness: i32,
    #[serde(alias = "Acousticness")]
    pub acousticness: i32,
    #[serde(alias = "Instrumentalness")]
    pub instrumentalness: i32,
    #[serde(alias = "Good for Party")]
    #[serde(with = "NumerToBool")]
    pub good_for_party: bool,
    #[serde(with = "NumerToBool")]
    #[serde(alias = "Good for Work/Study")]
    pub good_for_work: bool,
    #[serde(with = "NumerToBool")]
    #[serde(alias = "Good for Relaxation/Meditation")]
    pub good_for_relaxation: bool,
    #[serde(with = "NumerToBool")]
    #[serde(alias = "Good for Exercise")]
    pub good_for_exercise: bool,
    #[serde(with = "NumerToBool")]
    #[serde(alias = "Good for Running")]
    pub good_for_running: bool,
    #[serde(with = "NumerToBool")]
    #[serde(alias = "Good for Yoga/Stretching")]
    pub good_for_yoga: bool,
    #[serde(with = "NumerToBool")]
    #[serde(alias = "Good for Driving")]
    pub good_for_driving: bool,
    #[serde(with = "NumerToBool")]
    #[serde(alias = "Good for Social Gatherings")]
    pub good_for_social_gatherings: bool,
    #[serde(with = "NumerToBool")]
    #[serde(alias = "Good for Morning Routine")]
    pub good_for_morning_routine: bool,
    #[serde(alias = "Similar Artist 1")]
    pub similar_artist_1: String,
    #[serde(alias = "Similar Song 1")]
    pub similar_song_1: String,
    #[serde(alias = "Similarity Score 1")]
    pub similarity_score_1: f64,
    #[serde(alias = "Similar Artist 2")]
    pub similar_artist_2: String,
    #[serde(alias = "Similar Song 2")]
    pub similar_song_2: String,
    #[serde(alias = "Similarity Score 2")]
    pub similarity_score_2: f64,
    #[serde(alias = "Similar Artist 3")]
    pub similar_artist_3: String,
    #[serde(alias = "Similar Song 3")]
    pub similar_song_3: String,
    #[serde(alias = "Similarity Score 3")]
    pub similarity_score_3: f64,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub unique_id: String,
}

#[derive(Queryable,Debug)]
#[diesel(primary_key(id))]
pub struct SongInformationBase {
    pub song_id: Option<i32>,
    pub artist: String,
    pub song: String,
    pub text: String,
    pub length: String,
    pub emotion: String,
    pub genre: String,
    pub album: String,
    pub release_date: String,
    pub key: String,
    pub tempo: f64,
    pub loudness: f64,
    pub time_signature: String,
    pub explicit: String,
    pub popularity: i32,
    pub energy: i32,
    pub danceability: i32,
    pub positiveness: i32,
    pub speechiness: i32,
    pub liveness: i32,
    pub acousticness: i32,
    pub instrumentalness: i32,
    pub good_for_party: bool,
    pub good_for_work: bool,
    pub good_for_relaxation: bool,
    pub good_for_exercise: bool,
    pub good_for_running: bool,
    pub good_for_yoga: bool,
    pub good_for_driving: bool,
    pub good_for_social_gatherings: bool,
    pub good_for_morning_routine: bool,
    pub similar_artist_1: String,
    pub similar_song_1: String,
    pub similarity_score_1: f64,
    pub similar_artist_2: String,
    pub similar_song_2: String,
    pub similarity_score_2: f64,
    pub similar_artist_3: String,
    pub similar_song_3: String,
    pub similarity_score_3: f64,
    pub unique_id: String,
    pub created_at: Option<SystemTime>
}

impl UniqueId for SongInformation{
    fn get_unique_id(&self) -> String{
        let unique_cols = format!("{}{}{}", self.artist, self.song, self.genre);
        let result = Sha256::digest(unique_cols.as_bytes());
        return format!("{:X}", result)
    }
}

mod NumerToBool{
    use serde::{self, Deserialize, Serializer, Deserializer};
    pub fn serialize<S>(
        val: bool,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let new_val = match val{
           false => 0,
            true =>  1,
            _ => panic!("value out of ra")
        };
        serializer.serialize_i32(new_val)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<bool, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        //let  boolean_val= i32::deserialize(deserializer);

        match i32::deserialize(deserializer)?{
            0=>Ok(false),
            1=>Ok(true),
            _=>Err(serde::de::Error::custom("failed"))
        }
        //let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        //return binary
    }
}