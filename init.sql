-- Adminer 4.6.3 PostgreSQL dump

CREATE SEQUENCE categories_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."categories" (
    "id" integer DEFAULT nextval('categories_id_seq') NOT NULL,
    "name" text NOT NULL
) WITH (oids = false);

INSERT INTO "categories" ("id", "name") VALUES
(1,	'drinks');

CREATE SEQUENCE history_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START  CACHE 1;

CREATE TABLE "public"."history" (
    "times" integer NOT NULL,
    "item" text NOT NULL,
    "id" integer DEFAULT nextval('history_id_seq') NOT NULL,
    "year" date DEFAULT now() NOT NULL
) WITH (oids = false);


CREATE SEQUENCE items_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 3 CACHE 1;

CREATE TABLE "public"."items" (
    "id" integer DEFAULT nextval('items_id_seq') NOT NULL,
    "name" text NOT NULL,
    "price" real NOT NULL,
    "category_id" integer NOT NULL,
    "stock_id" integer,
    "description" text
) WITH (oids = false);


CREATE SEQUENCE orders_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 9 CACHE 1;

CREATE TABLE "public"."orders" (
    "id" integer DEFAULT nextval('orders_id_seq') NOT NULL,
    "item_id" integer NOT NULL,
    "table_id" integer NOT NULL,
    "quantity" integer NOT NULL,
    "time" timestamp DEFAULT now() NOT NULL
) WITH (oids = false);


CREATE SEQUENCE stock_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."stock" (
    "item_id" integer NOT NULL,
    "count" integer NOT NULL,
    "description" text NOT NULL,
    "place" text NOT NULL,
    "name" text NOT NULL,
    "id" integer DEFAULT nextval('stock_id_seq') NOT NULL
) WITH (oids = false);


CREATE SEQUENCE tables_id_seq1 INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 838 CACHE 1;

CREATE TABLE "public"."tables" (
    "name" text NOT NULL,
    "id" integer DEFAULT nextval('tables_id_seq1') NOT NULL
) WITH (oids = false);


-- 2019-07-29 18:48:24.107366+00

