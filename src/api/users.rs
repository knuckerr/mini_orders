use crate::api::errors::ServiceError;
use crate::common::connection;
use crate::model::user;
use actix_web::web::{Query};
use actix_web::{web, Error, HttpResponse};
use futures::future::Future;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserQuery {
    id: i32,
}


pub fn get_friends(
    pool: web::Data<connection::PgPool>,
    user: Query<UserQuery>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || user::get_friends(&pool,user.id))
        .then(move |res| match res {
            Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
            Err(e) => {
                dbg!(e);
                Err(ServiceError::InternalServerError)
            }
        })
        .from_err()
}


