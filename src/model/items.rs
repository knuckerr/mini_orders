use crate::common::connection::{handle_error, PgPool};
use crate::common::share::Msg;
use chrono::prelude::*;
use failure::Error;
use postgres::rows::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSumary {
    table: Table,
    item_total: i32,
    total: f32,
    last_order: NaiveDateTime,
}

