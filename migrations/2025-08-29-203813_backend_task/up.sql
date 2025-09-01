-- Your SQL goes here
CREATE TABLE "spotify_schema"."backend_task"(
           "id" SERIAL PRIMARY KEY,
           "task_name" VARCHAR NOT NULL,
           "status" VARCHAR NOT NULL,
           "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP);