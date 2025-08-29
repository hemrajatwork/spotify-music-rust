-- Your SQL goes here
CREATE TABLE "spotify_schema"."song_youtube_detail"(
            "id" SERIAL PRIMARY KEY,
            "song_id" INT4 references song_information(song_id),
            "youtube_link" VARCHAR NOT NULL,
            "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP)