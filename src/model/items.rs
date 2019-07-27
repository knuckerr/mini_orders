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
    pub category: String,
    pub stock_id: i32,
    pub description: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItem {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub category_id: i32,
    pub stock_id: i32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewItem {
    pub name: String,
    pub price: f32,
    pub category_id: i32,
    pub stock_id: i32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableItem {
    pub id: i32,
    pub table_id:i32,
    pub name: String,
    pub price: f32,
    pub category: String,
    pub stock_id: i32,
    pub description: String,
    pub quantity: i32,
    pub order: NaiveDateTime,
}

pub fn item_to_struct(row: &Row) -> Result<Item, Error> {
    let id_ = handle_error("id", row)?;
    let name_ = handle_error("name", row)?;
    let price_ = handle_error("price", row)?;
    let stock_id_ = handle_error("stock_id", row)?;
    let description_ = handle_error("description", row)?;
    let category_ = handle_error("category", row)?;
    let sumary = Item {
        id: id_,
        name: name_,
        price: price_,
        stock_id: stock_id_,
        description: description_,
        category: category_,
    };
    Ok(sumary)
}

pub fn table_item_to_struct(row: &Row) -> Result<TableItem, Error> {
    let id_ = handle_error("id", row)?;
    let table_id_ = handle_error("table_id",row)?;
    let name_ = handle_error("name", row)?;
    let price_ = handle_error("price", row)?;
    let stock_id_ = handle_error("stock_id", row)?;
    let description_ = handle_error("description", row)?;
    let category_ = handle_error("category", row)?;
    let quantity_ = handle_error("quantity", row)?;
    let order_ = row.get("time");
    let sumary = TableItem {
        id: id_,
        table_id: table_id_,
        name: name_,
        price: price_,
        stock_id: stock_id_,
        description: description_,
        category: category_,
        quantity: quantity_,
        order: order_,
    };
    Ok(sumary)
}

pub fn get_items(conn: &PgPool) -> Result<Vec<Item>, Error> {
    let mut tables = Vec::new();
    let conn = conn.get()?;
    let query = r#"
    Select items.id,items.name,items.price,items.stock_id,
    items.description,categories.name as category
    FROM items
    INNER JOIN categories on categories.id = items.category_id
        "#;
    let rows = conn.query(query, &[])?;
    for row in rows.into_iter() {
        let table = item_to_struct(&row)?;
        tables.push(table)
    }
    Ok(tables)
}

pub fn get_table_items(conn: &PgPool, table_id: i32) -> Result<Vec<TableItem>, Error> {
    let mut tables = Vec::new();
    let conn = conn.get()?;
    let query = r#"
        Select items.id,items.name,items.price,items.stock_id,
    table_id,
    items.description,categories.name as category,
    Coalesce(quantity,0) as quantity,Coalesce(time,to_timestamp(0)::timestamp) as time
    FROM items
    INNER JOIN categories on categories.id = items.category_id
    inner join orders on orders.item_id = items.id
    WHERE orders.table_id = $1
        "#;
    let rows = conn.query(query, &[&table_id])?;
    for row in rows.into_iter() {
        let table = table_item_to_struct(&row)?;
        tables.push(table)
    }
    Ok(tables)
}

pub fn new_item(conn: &PgPool, item: &NewItem) -> Result<Msg, Error> {
    let conn = conn.get()?;
    let query = r#"INSERT INTO "items" ("name", "price", "category_id", "stock_id", "description")
    VALUES ($1,$2,$3, NULL,$4);
    "#;
    conn.execute(
        query,
        &[
            &item.name,
            &item.price,
            &item.category_id,
            &item.description,
        ],
    )?;
    let msg = Msg {
        msg: "Insert item was sucess".to_string(),
        query: "Insert item ".to_string(),
        status: 200,
    };
    Ok(msg)
}


pub fn update_item(conn: &PgPool, item: &UpdateItem) -> Result<Msg, Error> {
    let conn = conn.get()?;
    let query = r#" UPDATE items SET "name" = $1, "price"=$2, "category_id"=$3,"description" =$4
    WHERE item."id" = $5
    "#;
    conn.execute(
        query,
        &[
            &item.name,
            &item.price,
            &item.category_id,
            &item.description,
            &item.id,
        ],
    )?;
    let msg = Msg {
        msg: "Update item was sucess".to_string(),
        query: "Update item ".to_string(),
        status: 200,
    };
    Ok(msg)
}

pub fn del_item(conn: &PgPool, id: i32) -> Result<Msg, Error> {
    let conn = conn.get()?;
    let query = r#" DELETE FROM item WHERE item.id=$1"#;
    conn.execute(query, &[&id])?;
    let msg = Msg {
        msg: "Delete item was sucess".to_string(),
        query: "Delete item ".to_string(),
        status: 200,
    };
    Ok(msg)
}

pub fn del_table_item(conn: &PgPool, table_id: i32, item_id: i32) -> Result<Msg, Error> {
    let conn = conn.get()?;
    let query = r#" DELETE FROM orders WHERE item.id=$1 AND table_id=$2"#;
    conn.execute(query, &[&item_id, &table_id])?;
    let msg = Msg {
        msg: "Delete table item was sucess".to_string(),
        query: "Delete table item ".to_string(),
        status: 200,
    };
    Ok(msg)
}

pub fn del_items(conn: &PgPool) -> Result<Msg, Error> {
    let conn = conn.get()?;
    let query = r#" DELETE FROM item"#;
    conn.execute(query, &[])?;
    let msg = Msg {
        msg: "Delete item was sucess".to_string(),
        query: "Delete item ".to_string(),
        status: 200,
    };
    Ok(msg)
}
