-- Adminer 4.6.3 PostgreSQL dump

CREATE SEQUENCE users_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."users" (
    "id" integer DEFAULT nextval('users_id_seq') NOT NULL,
    "username" text NOT NULL,
    "email" text NOT NULL,
    "avatar_img" text,
    "social_urls" jsonb DEFAULT '{"twiter": "", "facebook": "", "instagram": ""}',
    "description" text,
    "show_friends" boolean DEFAULT true,
    "show_places" boolean DEFAULT false,
    "public" boolean DEFAULT false,
    "active" boolean DEFAULT true NOT NULL,
    CONSTRAINT "users_id" UNIQUE ("id")
) WITH (oids = false);

INSERT INTO "users" ("id", "username", "email", "avatar_img", "social_urls", "description", "show_friends", "show_places", "public", "active") VALUES
(1,	'knucker',	'knucker@gmail.com',	NULL,	NULL,	'hello there i am john nice to meet you all',	't',	't',	't',	't'),
(2,	'test',	'test@gmail.com',	NULL,	NULL,	'hello there i am john nice to meet you all',	't',	't',	't',	't');

CREATE SEQUENCE favorite_places_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."favorite_places" (
    "id" integer DEFAULT nextval('favorite_places_id_seq') NOT NULL,
    "latitude" real NOT NULL,
    "longitude" real NOT NULL,
    "name" text NOT NULL,
    "user_id" integer NOT NULL,
    "photo_url" text,
    CONSTRAINT "favorite_places_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(id) NOT DEFERRABLE
) WITH (oids = false);

INSERT INTO "favorite_places" ("id", "latitude", "longitude", "name", "user_id", "photo_url") VALUES
(1,	36.4308,	28.2223,	'my home',	1,	NULL);

CREATE SEQUENCE followers_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."followers" (
    "id" integer DEFAULT nextval('followers_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "follower_id" integer NOT NULL,
    CONSTRAINT "followers_follower_id_fkey" FOREIGN KEY (follower_id) REFERENCES users(id) NOT DEFERRABLE
) WITH (oids = false);


CREATE SEQUENCE friend_request_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."friend_request" (
    "id" integer DEFAULT nextval('friend_request_id_seq') NOT NULL,
    "accept" text NOT NULL,
    "request_id" integer NOT NULL,
    "user_id" integer NOT NULL,
    "msg" text DEFAULT 'Let''s be friends' NOT NULL,
    "when" timestamp DEFAULT now() NOT NULL,
    CONSTRAINT "friend_request_request_id_fkey" FOREIGN KEY (request_id) REFERENCES users(id) NOT DEFERRABLE
) WITH (oids = false);


CREATE SEQUENCE friends_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."friends" (
    "id" integer DEFAULT nextval('friends_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "friend_id" integer NOT NULL,
    CONSTRAINT "friends_friend_id_fkey" FOREIGN KEY (friend_id) REFERENCES users(id) NOT DEFERRABLE
) WITH (oids = false);

INSERT INTO "friends" ("id", "user_id", "friend_id") VALUES
(1,	1,	2);

CREATE SEQUENCE hearts_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."hearts" (
    "id" integer DEFAULT nextval('hearts_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "heart_id" integer NOT NULL,
    CONSTRAINT "hearts_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(id) NOT DEFERRABLE
) WITH (oids = false);


CREATE SEQUENCE right_now_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."right_now" (
    "id" integer DEFAULT nextval('right_now_id_seq') NOT NULL,
    "latitude" numeric NOT NULL,
    "longitude" integer NOT NULL,
    "msg" text NOT NULL,
    "user_id" integer NOT NULL,
    "time_delete" timestamp NOT NULL,
    "photo_url" text,
    "only_friends" boolean DEFAULT true NOT NULL,
    "all_friends" boolean DEFAULT true NOT NULL,
    "select_friends" jsonb,
    CONSTRAINT "right_now_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(id) NOT DEFERRABLE
) WITH (oids = false);



-- 2019-07-06 12:34:22.559623+00

