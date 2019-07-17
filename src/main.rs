#![allow(dead_code)]
#[macro_use]
extern crate percent_encoding;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

extern crate env_logger;
extern crate log;

mod api;
mod common;
mod model;
use common::connection::Database;
use std::io;

fn main() -> io::Result<()> {
    let db = Database::new("root", "root", "common", "localhost", 5432);
    let pool = db.connection().expect("Cannot connect to database");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let app = move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::resource("/tables").route(web::get().to_async(api::table::get_tables)))
            .service(web::resource("/table").route(web::get().to_async(api::table::get_table)))
            .service(
                web::resource("/sumary_tables")
                    .route(web::get().to_async(api::table::sumary_tables)),
            )
            .service(web::resource("/new_table").route(web::post().to_async(api::table::new_table)))
            .service(web::resource("/del_table").route(web::post().to_async(api::table::del_table)))
            .service(web::resource("/update_table").route(web::post().to_async(api::table::update_table)))
            .service(
                web::resource("/range_table").route(web::post().to_async(api::table::range_table)),
            )
    };
    HttpServer::new(app).bind("localhost:5000")?.run()
}
