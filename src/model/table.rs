use crate::common::connection::{handle_error, PgPool};
use chrono::prelude::*;
use failure::Error;
use postgres::rows::Row;
use serde::{Deserialize, Serialize};
use crate::common::share::Msg;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSumary {
    table: Table,
    item_total: i32,
    total: f32,
    last_order: NaiveDateTime,
}

pub fn table_to_struct(row: &Row) -> Result<Table, Error> {
    let id = handle_error("id", row)?;
    let name = handle_error("name", row)?;
    Ok(Table { id: id, name: name })
}

pub fn get_all_tables(conn: &PgPool) -> Result<Vec<Table>, Error> {
    let mut users = Vec::new();
    let conn = conn.get()?;
    let query = r#"SELECT tables.* FROM tables
        "#;
    let rows = conn.query(query, &[])?;
    for row in rows.into_iter() {
        let user = table_to_struct(&row)?;
        users.push(user)
    }
    Ok(users)
}

pub fn table_sumary_struct(row: &Row) -> Result<TableSumary, Error> {
    let table = table_to_struct(row)?;
    let total = handle_error("total", row)?;
    let item_total = handle_error("item_total", row)?;
    let last_order = row.get("last_order");
    let sumary = TableSumary {
        table: table,
        item_total: item_total,
        total: total,
        last_order: last_order,
    };
    Ok(sumary)
}

pub fn get_tables_sumary(conn: &PgPool) -> Result<Vec<TableSumary>, Error> {
    let mut tables = Vec::new();
    let conn = conn.get()?;
    let query = r#"
        select tables."name",tables."id",
    Coalesce(SUM (orders."quantity" * items."price"),0)::real as total ,
    Coalesce(SUM(orders."quantity")::int,0) as item_total,
    Coalesce(MAX (orders."time"),to_timestamp(0)::timestamp) as last_order
    from tables
    full outer join orders on orders."table_id" = tables."id"
    full outer join items on items."id" = orders."item_id"
    group by tables."name",tables."id"
    ORDER BY tables."name"
    "#;
    let rows = conn.query(query, &[])?;
    for row in rows.into_iter() {
        let table = table_sumary_struct(&row)?;
        tables.push(table)
    }
    Ok(tables)
}

pub fn new_table(conn:&PgPool,name:&str) -> Result<Msg,Error> {
    let conn = conn.get()?;
    let exist = conn.query("SELECT * FROM tables WHERE name=$1",&[&name])?;
    if exist.into_iter().len() >= 1 {
        let msg = Msg{msg:"ERROR Aready EXIST".to_string(),query:"Insert".to_string(),status:400};
        Ok(msg)

    }else{
        conn.execute(r#" INSERT INTO tables(name) values($1)"#,&[&name])?;
        let msg = Msg{msg:"Success".to_string(),query:"Insert".to_string(),status:200};
        Ok(msg)
    }
}
