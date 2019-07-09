use crate::common::connection::PgPool;
use failure::{err_msg, Error};
use postgres::rows::Row;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
   pub  id: i32,
   pub  username: String,
   pub  email: String,
   pub  avatar_img: String,
   pub  social_urls: Social,
   pub  description: String,
   pub  show_friends: bool,
   pub  show_places: bool,
   pub  public: bool,
   pub  active: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
   pub  users: Vec<User>,
}


#[derive(Debug,Serialize, Deserialize)]
pub struct Social {
  twiter: String,
  facebook: String,
  instagram: String,
}



pub fn handle_error_str(key:&str,row:&Row) -> Result<String,Error> {
    let err = format!("{} not exist",key);
    let data = row.get_opt(key)
        .ok_or(err_msg(err))?
        .unwrap_or(String::from("None"));
    Ok(data)
}


pub fn handle_error_bool(key:&str,row:&Row) -> Result<bool,Error> {
    let err = format!("{} not exist",key);
    let data = row.get_opt(key)
        .ok_or(err_msg(err))?
        .unwrap_or(false);
    Ok(data)
}



pub fn rows_to_struct(row: Row) -> Result<User, Error> {
    let id = row
        .get_opt("id")
        .ok_or(err_msg("id key not exist"))?
        .unwrap_or(0);
    let username = handle_error_str("username",&row)?;
    let email = handle_error_str("email",&row)?;
    let avatar_img = handle_error_str("avatar_img",&row)?;
    let social_urls = row
        .get_opt("social_urls")
        .ok_or(err_msg("social_urls key not exist"))?
        .unwrap_or(json!({"twiter": "", "facebook": "", "instagram": ""}));
    let socials:Social = serde_json::from_value(social_urls)?;
    let description = handle_error_str("description",&row)?;
    let show_friends = handle_error_bool("show_friends",&row)?;
    let show_places = handle_error_bool("show_places",&row)?;
    let public = handle_error_bool("public",&row)?;
    let active = handle_error_bool("active",&row)?;
    Ok(User {
        id: id,
        username: username,
        email: email,
        avatar_img: avatar_img,
        social_urls: socials,
        description: description,
        show_friends: show_friends,
        show_places: show_places,
        public: public,
        active: active,
    })
}

pub fn get_friends(conn: &PgPool,id:i32) -> Result<Vec<User>, Error> {
    let mut users = Vec::new();
    let conn = conn.get()?;
    let query = r#"SELECT users.* FROM users
        inner join friends on friend_id = users.id 
        where user_id = $1
        and users.id != $2
        "#;
    let rows = conn.query(query, &[&id,&id])?;
    for row in rows.into_iter() {
        let user = rows_to_struct(row)?;
        users.push(user)
    }
    Ok(users)
}


pub fn unseen_friend_requests(conn: &PgPool,id:i32) -> Result<Vec<User>, Error> {
    let mut users = Vec::new();
    let conn = conn.get()?;
    let query = r#"
    select users.* from users
    inner join friend_request on friend_request.request_id = users.id
    where friend_request.request_id NOT IN(select friend_id from seen_requests)
    and friend_request.user_id = $1
        "#;
    let rows = conn.query(query, &[&id])?;
    for row in rows.into_iter() {
        let user = rows_to_struct(row)?;
        users.push(user)
    }
    Ok(users)
}

pub fn mark_seen_request(conn:&PgPool,users:Vec<User>,id:i32) -> Result<(),Error>{
    if users.len() > 0 {
        let conn = conn.get()?;
        for user in users {
            let query = r#" INSERT INTO seen_requests(friend_id,user_id) VALUES($1,$2)"#;
            conn.execute(query,&[&user.id, &id])?;
        }
    }
    Ok(())
}
