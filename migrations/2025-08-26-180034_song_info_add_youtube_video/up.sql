-- Your SQL goes here
ALTER TABLE "spotify_schema"."song_information"
    ADD COLUMN "youtube_video" BOOLEAN NOT NULL DEFAULT FALSE;
