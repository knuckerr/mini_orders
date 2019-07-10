use percent_encoding::{utf8_percent_encode, SIMPLE_ENCODE_SET};
use std::fs::File;
use failure::{Error,err_msg};
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json;
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use postgres::rows::Row;


define_encode_set! {
    /// This encode set is used in the URL parser for query strings.
    pub QUERY_ENCODE_SET = [SIMPLE_ENCODE_SET] | {' ', '"', '#', '<', '>','%','&','@'}
}

pub type PgPool = Pool<PostgresConnectionManager>;

#[derive(Debug, Clone, Copy,Serialize, Deserialize)]
pub struct Database<'a> {
    pub user: &'a str,
    pub pass: &'a str,
    pub db: &'a str,
    pub ip: &'a str,
    pub port: i32,
}

impl<'a> Database<'a> {
    pub fn new(
        user: &'a str,
        pass: &'a str,
        db: &'a str,
        ip: &'a str,
        port: i32,
    ) -> Database<'a> {
        Database {
            user: user,
            pass: pass,
            db: db,
            ip: ip,
            port: port,
        }
    }
    pub fn connection(&self) -> Result<PgPool,Error> {
        let pass = utf8_percent_encode(self.pass, QUERY_ENCODE_SET).to_string();
        let user = utf8_percent_encode(self.user, QUERY_ENCODE_SET).to_string();
        let url_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, pass, self.ip, self.port, self.db
        );
        let config = PostgresConnectionManager::new(url_string,r2d2_postgres::TlsMode::None)?;
        let pool = Pool::builder().build(config)?;
        Ok(pool)
    }
    pub fn json_connection(path:&str)->Result<PgPool,Error>{
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let db : Database =  serde_json::from_str(&data)?;
        let con = db.connection();
        con

    }
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


pub fn handle_error_i32(key:&str,row:&Row) -> Result<i32,Error> {
    let err = format!("{} not exist",key);
    let data = row.get_opt(key)
        .ok_or(err_msg(err))?
        .unwrap_or(0);
    Ok(data)
}
