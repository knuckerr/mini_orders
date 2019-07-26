use crate::api::errors::ServiceError;
use crate::common::connection;
use crate::model::table;
use actix_web::{web, Error, HttpResponse};
use failure::err_msg;
use futures::future::Future;
use std::collections::HashMap;

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

pub fn get_table(
    pool: web::Data<connection::PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let name = query.get("name").ok_or_else(||err_msg("missing the name value"))?;
        table::get_table_sumary(&pool, &name)
    })
    .then(move |res| match res {
        Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
        Err(e) => {
            dbg!(e);
            Err(ServiceError::BadRequest(
                "missing the name value".to_string(),
            ))
        }
    })
    .from_err()
}

pub fn new_table(
    pool: web::Data<connection::PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let name = query.get("name").ok_or_else(||err_msg("missing the name value"))?;
        table::new_table(&pool, &name)
    })
    .then(move |res| match res {
        Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
        Err(e) => {
            dbg!(e);
            Err(ServiceError::BadRequest(
                "missing the name value".to_string(),
            ))
        }
    })
    .from_err()
}

pub fn del_table(
    pool: web::Data<connection::PgPool>,
    query: web::Json<Vec<i32>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        table::del_table(&pool,&query)
    })
    .then(move |res| match res {
        Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
        Err(e) => {
            dbg!(e);
            Err(ServiceError::BadRequest(
                "missing the name value".to_string(),
            ))
        }
    })
    .from_err()
}


pub fn clear_orders(
    pool: web::Data<connection::PgPool>,
    query: web::Json<Vec<i32>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        table::clear_orders(&pool,&query)
    })
    .then(move |res| match res {
        Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
        Err(e) => {
            dbg!(e);
            Err(ServiceError::BadRequest(
                "missing the name value".to_string(),
            ))
        }
    })
    .from_err()
}

pub fn update_table(
    pool: web::Data<connection::PgPool>,
    table: web::Json<table::Table>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || table::update_table(&pool, &table))
        .then(move |res| match res {
            Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
            Err(e) => {
                dbg!(e);
                Err(ServiceError::BadRequest(
                    "missing the name value".to_string(),
                ))
            }
        })
        .from_err()
}

pub fn range_table(
    pool: web::Data<connection::PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let from: i32 = query
            .get("from")
            .ok_or_else(|| err_msg("missing the from value"))?
            .parse()?;
        let to: i32 = query
            .get("to")
            .ok_or_else(|| err_msg("missing the to value"))?
            .parse()?;
        table::generate_table(&pool, from, to)
    })
    .then(move |res| match res {
        Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
        Err(e) => {
            dbg!(e);
            Err(ServiceError::BadRequest(
                "missing the from & to  value".to_string(),
            ))
        }
    })
    .from_err()
}
