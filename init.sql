-- Adminer 4.6.3 PostgreSQL dump

CREATE SEQUENCE categories_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."categories" (
    "id" integer DEFAULT nextval('categories_id_seq') NOT NULL,
    "name" text NOT NULL
) WITH (oids = false);

INSERT INTO "categories" ("id", "name") VALUES
(1,	'drinks');

CREATE SEQUENCE items_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 2 CACHE 1;

CREATE TABLE "public"."items" (
    "id" integer DEFAULT nextval('items_id_seq') NOT NULL,
    "name" text NOT NULL,
    "price" real NOT NULL,
    "category_id" integer NOT NULL,
    "stock_id" integer
) WITH (oids = false);

INSERT INTO "items" ("id", "name", "price", "category_id", "stock_id") VALUES
(1,	'coca cola',	2.7,	1,	NULL),
(2,	'wine',	4,	1,	NULL);

CREATE SEQUENCE orders_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 3 CACHE 1;

CREATE TABLE "public"."orders" (
    "id" integer DEFAULT nextval('orders_id_seq') NOT NULL,
    "item_id" integer NOT NULL,
    "table_id" integer NOT NULL,
    "quantity" integer NOT NULL,
    "time" timestamp DEFAULT now() NOT NULL
) WITH (oids = false);

INSERT INTO "orders" ("id", "item_id", "table_id", "quantity", "time") VALUES
(1,	1,	1,	2,	'2019-07-16 11:40:06.894758'),
(2,	2,	1,	4,	'2019-07-16 11:42:03.390676'),
(3,	2,	2,	10,	'2019-07-16 14:09:27.848621');

CREATE SEQUENCE tables_id_seq1 INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 6 CACHE 1;

CREATE TABLE "public"."tables" (
    "name" text NOT NULL,
    "id" integer DEFAULT nextval('tables_id_seq1') NOT NULL
) WITH (oids = false);

INSERT INTO "tables" ("name", "id") VALUES
('1',	1),
('2',	2),
('3',	3),
('4',	4),
('5',	6);

-- 2019-07-16 18:39:39.135904+00

