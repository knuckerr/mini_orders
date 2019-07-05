#![allow(dead_code)]
#[macro_use]
extern crate percent_encoding;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

extern crate log;
extern crate env_logger;

mod api;
mod common;
mod model;
use common::connection::Database;
use std::io;

fn main()-> io::Result<()> {
    let db = Database::new("root", "root", "common", "localhost", 5432);
    let pool = db.connection().expect("Cannot connect to database");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let app = move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/friends").route(web::get().to_async(api::users::get_friends)),
            )
    };
    HttpServer::new(app)
        .bind("localhost:8088")?
        .run()
}
