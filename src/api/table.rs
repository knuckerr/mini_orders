use crate::api::errors::ServiceError;
use crate::common::connection;
use crate::model::table;
use actix_web::{web, Error, HttpResponse};
use futures::future::Future;
use std::collections::HashMap;
use failure::err_msg;

pub fn get_tables(
    pool: web::Data<connection::PgPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || table::get_all_tables(&pool))
        .then(move |res| match res {
            Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
            Err(e) => {
                dbg!(e);
                Err(ServiceError::InternalServerError)
            }
        })
        .from_err()
}

pub fn sumary_tables(
    pool: web::Data<connection::PgPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || table::get_tables_sumary(&pool))
        .then(move |res| match res {
            Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
            Err(e) => {
                dbg!(e);
                Err(ServiceError::InternalServerError)
            }
        })
        .from_err()
}

pub fn new_table(
    pool: web::Data<connection::PgPool>,
    query: web::Query<HashMap<String, String>>
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let name = query.get("name").ok_or(err_msg("missing the name value"))?;
        table::new_table(&pool,&name)
    })
        .then(move |res| match res {
            Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
            Err(e) => {
                dbg!(e);
                Err(ServiceError::BadRequest("missing the name value".to_string()))
            }
        })
        .from_err()
}
