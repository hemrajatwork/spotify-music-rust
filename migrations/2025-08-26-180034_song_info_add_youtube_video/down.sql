-- This file should undo anything in `up.sql`
-- Your SQL goes here
ALTER TABLE "spotify_schema"."song_information"
    DROP COLUMN "youtube_video";
